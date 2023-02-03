//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "../library/Bn254.sol";
import "../library/TranscriptProtocol.sol";

contract MockTranscript {
    using TranscriptProtocol for TranscriptProtocol.Transcript;

    function testTranscript(
        uint64 item1,
        Bn254.Fr memory item2,
        Bn254.G1Point memory item3
    ) external pure returns (uint256, uint256, uint256) {
        TranscriptProtocol.Transcript memory t = TranscriptProtocol.newTranscript();
        t.appendUint64(item1);
        Bn254.Fr memory a = t.challengeFr();
        t.appendFr(item2);
        Bn254.Fr memory b = t.challengeFr();
        t.appendG1(item3);
        Bn254.Fr memory c = t.challengeFr();
        return (a.value, b.value, c.value);
    }
}