// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";
import "./Domain.sol";
import "./KZGChecker.sol";
import "./TranscriptProtocol.sol";

library BalanceSumVerifier {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;
    using TranscriptProtocol for TranscriptProtocol.Transcript;

    struct Proof {
        // Evluations
        Bn254.Fr b;
        Bn254.Fr t;
        Bn254.Fr h1;
        Bn254.Fr h2;
        Bn254.Fr sNext;
        Bn254.Fr zNext;
        Bn254.Fr h1Next;
        Bn254.Fr h2Next;

        // Commitments
        Bn254.G1Point bCommit;
        Bn254.G1Point sCommit;
        Bn254.G1Point h1Commit;
        Bn254.G1Point h2Commit;
        Bn254.G1Point zCommit;
        Bn254.G1Point q1Commit;
        Bn254.G1Point q2Commit;
        Bn254.G1Point opening1;
        Bn254.G1Point opening2;
    }

    struct Challenges {
        Bn254.Fr gamma;
        Bn254.Fr z;
        Bn254.Fr lambda;
        Bn254.Fr[] deltas;
        Bn254.Fr[] etas;
    }

    // Precomputed [t(X)]
    function tCommit() internal pure returns (Bn254.G1Point memory) {
        return Bn254.G1Point(
            0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b,
            0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b
        );
    }
    
    function evaluateVanishingPoly(Bn254.Fr memory tau) internal view returns (Bn254.Fr memory) {
        Bn254.Fr memory tmp = tau.pow(Domain.SIZE);
        tmp.subAssign(Bn254.Fr(1));

        return tmp;
    }

    function evaluateFirstLagrangePoly(
        Bn254.Fr memory tau,
        Bn254.Fr memory zh
    ) internal view returns (Bn254.Fr memory) {
        Bn254.Fr memory tmp = tau.sub(Bn254.Fr(1));
        tmp.mulAssign(Bn254.Fr(Domain.SIZE));
        tmp.inverseAssign();
        tmp.mulAssign(zh);

        return tmp;
    }

    function evaluateLastLagrangePoly(
        Bn254.Fr memory tau,
        Bn254.Fr memory zh
    ) internal view returns (Bn254.Fr memory) {
        Bn254.Fr memory omegaInv = Domain.domainGeneratorInv();
        Bn254.Fr memory tmp = tau.sub(omegaInv);
        tmp.mulAssign(Bn254.Fr(Domain.SIZE));
        tmp.inverseAssign();
        tmp.mulAssign(zh);
        tmp.mulAssign(omegaInv);

        return tmp;
    }

    function computeEvaluation1(
        Proof memory proof,
        Challenges memory challenges,
        Bn254.Fr memory firstLagEval,
        Bn254.Fr memory lastLagEval,
        Bn254.Fr memory m
    ) internal pure returns (Bn254.Fr memory) {
        Bn254.Fr memory evaluation = Bn254.Fr(0).sub(proof.sNext);
        Bn254.Fr memory tmp = m.mul(firstLagEval);
        evaluation.subAssign(tmp);

        tmp.copyFromFr(challenges.gamma);
        tmp.addAssign(proof.h1);
        tmp.mulAssign(proof.zNext);
        tmp.mulAssign(challenges.gamma);
        tmp.mulAssign(challenges.deltas[0]);
        evaluation.addAssign(tmp);

        tmp.copyFromFr(firstLagEval);
        tmp.mulAssign(challenges.deltas[1]);
        evaluation.addAssign(tmp);

        Bn254.Fr memory lastLagEvalSubOne = lastLagEval.sub(Bn254.Fr(1));
        tmp.copyFromFr(proof.h1Next);
        tmp.subAssign(proof.h1);
        tmp.subAssign(Bn254.Fr(1));
        tmp.mulAssign(proof.h1Next);
        tmp.mulAssign(lastLagEvalSubOne);
        tmp.mulAssign(challenges.deltas[2]);
        evaluation.subAssign(tmp);

        tmp.copyFromFr(proof.h2Next);  
        tmp.subAssign(proof.h2);
        tmp.subAssign(Bn254.Fr(1));
        tmp.mulAssign(proof.h2Next);
        tmp.mulAssign(lastLagEvalSubOne);
        tmp.mulAssign(challenges.deltas[3]);
        evaluation.subAssign(tmp);

        tmp.copyFromFr(proof.h2Next);
        tmp.subAssign(proof.h1);
        tmp.subAssign(Bn254.Fr(1));
        tmp.mulAssign(proof.h2Next);
        tmp.mulAssign(lastLagEval);
        tmp.mulAssign(challenges.deltas[4]);
        evaluation.subAssign(tmp);

        tmp.copyFromFr(Bn254.Fr(Domain.SIZE - 1));
        tmp.mulAssign(lastLagEval);
        tmp.mulAssign(challenges.deltas[6]);
        evaluation.addAssign(tmp);

        tmp.copyFromFr(proof.b);
        tmp.mulAssign(challenges.etas[0]);
        evaluation.addAssign(tmp);

        tmp.copyFromFr(proof.t);
        tmp.mulAssign(challenges.etas[1]);
        evaluation.addAssign(tmp);

        tmp.copyFromFr(proof.h1);
        tmp.mulAssign(challenges.etas[2]);
        evaluation.addAssign(tmp);

        tmp.copyFromFr(proof.h2);
        tmp.mulAssign(challenges.etas[3]);
        evaluation.addAssign(tmp);
        
        return evaluation;
    }

    function computeEvaluation2(
        Proof memory proof,
        Challenges memory challenges
    ) pure internal returns (Bn254.Fr memory) {
        Bn254.Fr memory evaluation = proof.sNext.cloneFr();
        Bn254.Fr memory tmp = proof.zNext.mul(challenges.etas[0]);
        evaluation.addAssign(tmp);
        tmp.copyFromFr(proof.h1Next);
        tmp.mulAssign(challenges.etas[1]);
        evaluation.addAssign(tmp);
        tmp.copyFromFr(proof.h2Next);
        tmp.mulAssign(challenges.etas[2]);
        evaluation.addAssign(tmp);

        return evaluation;
    }

    function linearisationCommitments1(
        Proof memory proof,
        Challenges memory challenges,
        Bn254.Fr memory zh,
        Bn254.Fr memory firstLagEval,
        Bn254.Fr memory lastLagEval
    ) internal view returns (Bn254.G1Point memory) {
        // -[s(X)]
        Bn254.G1Point memory commitment = proof.sCommit.pointNegate();

        // eta * [t(X)]
        Bn254.G1Point memory tmpPoint = tCommit().pointMul(challenges.etas[0]);
        commitment.pointAddAssign(tmpPoint);

        // (eta^2 - 1) * [B(X)]
        Bn254.Fr memory scalar = challenges.etas[1].sub(Bn254.Fr(1));
        tmpPoint.copyFromG1(proof.bCommit);
        tmpPoint.pointMul(scalar);
        commitment.pointAddAssign(tmpPoint);

        // scalar = (gamma + b) * (gamma + t) * delta + firstLag * delta^2
        Bn254.Fr memory tmp = challenges.gamma.add(proof.b);
        scalar.copyFromFr(tmp);
        tmp.copyFromFr(challenges.gamma);
        tmp.addAssign(proof.t);
        scalar.mulAssign(tmp);
        scalar.mulAssign(challenges.deltas[0]);
        tmp.copyFromFr(firstLagEval);
        tmp.mulAssign(challenges.deltas[1]);
        scalar.addAssign(tmp);
        // scalar * [z(X)]
        tmpPoint.copyFromG1(proof.zCommit);
        tmpPoint.pointMulAssign(scalar);
        commitment.pointAddAssign(tmpPoint);

        // scalar = firstLag * delta^6 + eta^3
        //          - (h1Next - h1 - 1) * (lastLag - 1) * delta^3
        //          - (h2Next - h1 - 1) * lastLag * delta^5
        Bn254.Fr memory h1PlusOne = proof.h1.add(Bn254.Fr(1));
        Bn254.Fr memory lastLagEvalSubOne = lastLagEval.sub(Bn254.Fr(1));
        scalar.copyFromFr(firstLagEval);
        scalar.mulAssign(challenges.deltas[5]);
        scalar.addAssign(challenges.etas[2]);
        tmp.copyFromFr(proof.h1Next);
        tmp.subAssign(h1PlusOne);
        tmp.mulAssign(lastLagEvalSubOne);
        tmp.mulAssign(challenges.deltas[2]);
        scalar.subAssign(tmp);
        tmp.copyFromFr(proof.h2Next);
        tmp.subAssign(h1PlusOne);
        tmp.mulAssign(lastLagEval);
        tmp.mulAssign(challenges.deltas[4]);
        scalar.subAssign(tmp);
        // scalar * [h1(X)]
        tmpPoint.copyFromG1(proof.h1Commit);
        tmpPoint.pointMulAssign(scalar);
        commitment.pointAddAssign(tmpPoint);

        // scalar = lastLag * delta^7 + eta^4
        //          - zNext * (gamma + h1) * delta
        //          - (h2Next - h2 - 1) * (lastLag - 1) * delta^4
        scalar.copyFromFr(lastLagEval);
        scalar.mulAssign(challenges.deltas[6]);
        scalar.addAssign(challenges.etas[3]);
        tmp.copyFromFr(challenges.gamma);
        tmp.addAssign(proof.h1);
        tmp.mulAssign(proof.zNext);
        tmp.mulAssign(challenges.deltas[0]);
        scalar.subAssign(tmp);
        tmp.copyFromFr(proof.h2Next);
        tmp.subAssign(proof.h2);
        tmp.subAssign(Bn254.Fr(1));
        tmp.mulAssign(lastLagEvalSubOne);
        tmp.mulAssign(challenges.deltas[3]);
        scalar.subAssign(tmp);
        // scalar * [h2(X)]
        tmpPoint.copyFromG1(proof.h2Commit);
        tmpPoint.pointMulAssign(scalar);
        commitment.pointAddAssign(tmpPoint);

        // -zh * [q1(X)]
        tmpPoint.copyFromG1(proof.q1Commit);
        tmpPoint.pointMulAssign(zh);
        commitment.pointSubAssign(tmpPoint);

        // scalar = -zh * (zh + 1) * z^3
        scalar.copyFromFr(zh);
        scalar.addAssign(Bn254.Fr(1));
        scalar.mulAssign(zh);
        scalar.mulAssign(challenges.z);
        scalar.mulAssign(challenges.z);
        scalar.mulAssign(challenges.z);
        // scalar * [q2(X)]
        tmpPoint.copyFromG1(proof.q2Commit);
        tmpPoint.pointMulAssign(scalar);
        commitment.pointSubAssign(tmpPoint);

        return commitment;
    }

    function linearisationCommitments2(
        Proof memory proof,
        Challenges memory challenges
    ) internal view returns (Bn254.G1Point memory) {
        // [S(X)]
        Bn254.G1Point memory commitment = proof.sCommit.cloneG1();

        // eta * [z(X)]
        Bn254.G1Point memory tmpPoint = proof.zCommit.pointMul(challenges.etas[0]);
        commitment.pointAddAssign(tmpPoint);

        // eta^2 * [h1(X)]
        tmpPoint.copyFromG1(proof.h1Commit);
        tmpPoint.pointMulAssign(challenges.etas[1]);
        commitment.pointAddAssign(tmpPoint);

        // eta^3 * [h2(X)]
        tmpPoint.copyFromG1(proof.h2Commit);
        tmpPoint.pointMulAssign(challenges.etas[2]);
        commitment.pointAddAssign(tmpPoint);

        return commitment;
    }

    function generateChallenges(
        Proof memory proof,
        Bn254.Fr memory m
    ) internal pure returns (Challenges memory) {
        // Initialize transcript
        TranscriptProtocol.Transcript memory transcript = TranscriptProtocol.newTranscript();
        transcript.appendUint256(Domain.SIZE);

        transcript.appendFr(m);
        transcript.appendG1(proof.bCommit);
        transcript.appendG1(proof.sCommit);
        transcript.appendG1(proof.h1Commit);
        transcript.appendG1(proof.h2Commit);
        // Compute challenge gamma
        Bn254.Fr memory gamma = transcript.challengeFr();

        transcript.appendG1(proof.zCommit);
        // Compute challenge delta
        Bn254.Fr memory delta = transcript.challengeFr();

        transcript.appendG1(proof.q1Commit);
        transcript.appendG1(proof.q2Commit);
        // Compute challenge z
        Bn254.Fr memory z = transcript.challengeFr();

        transcript.appendFr(proof.t);
        transcript.appendFr(proof.b);
        transcript.appendFr(proof.h1);
        transcript.appendFr(proof.h2);
        transcript.appendFr(proof.sNext);
        transcript.appendFr(proof.h1Next);
        transcript.appendFr(proof.h2Next);
        transcript.appendFr(proof.zNext);
        // Compute challenge eta
        Bn254.Fr memory eta = transcript.challengeFr();

        transcript.appendG1(proof.opening1);
        transcript.appendG1(proof.opening2);
        // Compute challenge lambda
        Bn254.Fr memory lambda = transcript.challengeFr();

        // Expand deltas vector
        Bn254.Fr[] memory deltas = new Bn254.Fr[](7);
        deltas[0].copyFromFr(delta);
        for (uint256 i = 1; i < 7; i++) {
            deltas[i].copyFromFr(deltas[i - 1].mul(delta));
        }
        // Expand etas vectors
        Bn254.Fr[] memory etas = new Bn254.Fr[](4);
        etas[0].copyFromFr(eta);
        for (uint256 i = 1; i < 7; i++) {
            deltas[i].copyFromFr(deltas[i - 1].mul(delta));
        }

        return Challenges(gamma, z, lambda, deltas, etas);
    }

    function verify(Proof memory proof, Bn254.Fr memory m) internal view returns (bool) {
        // Generate challenges via Fiat-Shamir algorithm
        Challenges memory challenges = generateChallenges(proof, m);

        // Compute vanishing polynomial evaluation
        Bn254.Fr memory zh = evaluateVanishingPoly(challenges.z);
        // Compute first Lagrange polynomial evaluation
        Bn254.Fr memory firstLagEval = evaluateFirstLagrangePoly(challenges.z, zh);
        // Compute last Lagrange polynomial evaluation
        Bn254.Fr memory lastLagEval = evaluateLastLagrangePoly(challenges.z, zh);

        // Compute evaluation 1
        Bn254.Fr memory evaluation1 = computeEvaluation1(
            proof,
            challenges,
            firstLagEval,
            lastLagEval,
            m
        );
        // Compute commitment 1
        Bn254.G1Point memory commitment1 = linearisationCommitments1(
            proof,
            challenges,
            zh,
            firstLagEval,
            lastLagEval
        );
        
        // Compute evaluation 2
        Bn254.Fr memory evaluation2 = computeEvaluation2(proof, challenges);
        // Compute commitment 2
        Bn254.G1Point memory commitment2 = linearisationCommitments2(proof, challenges);

        // KZG batch check
        Bn254.Fr[] memory points = new Bn254.Fr[](2);
        points[0].copyFromFr(challenges.z);
        points[1].copyFromFr(Domain.domainGenerator().mul(challenges.z));
        Bn254.Fr[] memory evals = new Bn254.Fr[](2);
        evals[0].copyFromFr(evaluation1);
        evals[1].copyFromFr(evaluation2);
        Bn254.G1Point[] memory openings = new Bn254.G1Point[](2);
        openings[0].copyFromG1(proof.opening1);
        openings[1].copyFromG1(proof.opening2);
        Bn254.G1Point[] memory commitments = new Bn254.G1Point[](2);
        commitments[0].copyFromG1(commitment1);
        commitments[1].copyFromG1(commitment2);

        return KZGChecker.batchCheck(
            challenges.lambda,
            points,
            evals,
            openings,
            commitments
        );
    }
}