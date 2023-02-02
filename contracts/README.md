# PoSol Verifying Contract

Goerli: 0x6ce2C11f53f1199685770BbFd2cCba5245D36B1b

```json
[
    {
      "inputs": [
        {
          "components": [
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "b",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "t",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "h1",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "h2",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "sNext",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "zNext",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "h1Next",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "value",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.Fr",
              "name": "h2Next",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "bCommit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "sCommit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "h1Commit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "h2Commit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "zCommit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "q1Commit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "q2Commit",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "opening1",
              "type": "tuple"
            },
            {
              "components": [
                {
                  "internalType": "uint256",
                  "name": "x",
                  "type": "uint256"
                },
                {
                  "internalType": "uint256",
                  "name": "y",
                  "type": "uint256"
                }
              ],
              "internalType": "struct Bn254.G1Point",
              "name": "opening2",
              "type": "tuple"
            }
          ],
          "internalType": "struct BalanceSumVerifier.Proof",
          "name": "proof",
          "type": "tuple"
        },
        {
          "components": [
            {
              "internalType": "uint256",
              "name": "value",
              "type": "uint256"
            }
          ],
          "internalType": "struct Bn254.Fr",
          "name": "balanceSum",
          "type": "tuple"
        }
    ],
    "name": "verifyBalanceSum",
    "outputs": [
      {
        "internalType": "bool",
        "name": "",
        "type": "bool"
      }
    ],
    "stateMutability": "view",
    "type": "function"
    }
]
```