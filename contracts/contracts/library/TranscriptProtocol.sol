// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library TranscriptProtocol {
    using Bn254 for Bn254.Fr;

    // flip                    0xe000000000000000000000000000000000000000000000000000000000000000;
    uint256 constant private FR_MASK = 0x1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

    uint32 constant private DST_0 = 0;
    uint32 constant private DST_1 = 1;
    uint32 constant private DST_CHALLENGE = 2;

    struct Transcript {
        bytes32 state0;
        bytes32 state1;
        uint32 counter;
    }

    function newTranscript() internal pure returns (Transcript memory t) {
        t.state0 = bytes32(0);
        t.state1 = bytes32(0);
        t.counter = 0;
    }

    function appendUint256(Transcript memory self, uint256 value) internal pure {
        bytes32 oldState = self.state0;
        self.state0 = keccak256(abi.encodePacked(DST_0, oldState, self.state1, value));
        self.state1 = keccak256(abi.encodePacked(DST_1, oldState, self.state1, value));
    }

    function appendFr(Transcript memory self, Bn254.Fr memory value) internal pure {
        appendUint256(self, value.value);
    }

    function appendG1(Transcript memory self, Bn254.G1Point memory p) internal pure {
        appendUint256(self, p.X);
        appendUint256(self, p.Y);
    }

    function challengeFr(Transcript memory self) internal pure returns (Bn254.Fr memory) {
        bytes32 query = keccak256(abi.encodePacked(DST_CHALLENGE, self.state0, self.state1, self.counter));
        self.counter += 1;
        return Bn254.Fr(uint256(query) & FR_MASK);
    }
}