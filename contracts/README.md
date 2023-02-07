# PoSol Verifying Contract

## Gas

```bash
·----------------------------------------------|---------------------------|---------------|-----------------------------·
|             Solc version: 0.8.4              ·  Optimizer enabled: true  ·  Runs: 10000  ·  Block limit: 30000000 gas  │
···············································|···························|···············|······························
|  Methods                                                                                                               │
···························|···················|·············|·············|···············|···············|··············
|  Contract                ·  Method           ·  Min        ·  Max        ·  Avg          ·  # calls      ·  usd (avg)  │
···························|···················|·············|·············|···············|···············|··············
|  TestBalanceSumVerifier  ·  testVerifyProof  ·          -  ·          -  ·       334219  ·            1  ·          -  │
···························|···················|·············|·············|···············|···············|··············
|  Deployments                                 ·                                           ·  % of limit   ·             │
···············································|·············|·············|···············|···············|··············
|  TestBalanceSumVerifier                      ·          -  ·          -  ·      2870751  ·        9.6 %  ·          -  │
·----------------------------------------------|-------------|-------------|---------------|---------------|-------------·
```

## Config

```json
{
  "url": "https://goerli.infura.io/v3/54d13b257eb94b2eab1c7875e0d8d301",
  "sender": "0xfc5ca0a6e373966a3188c9abc191713614e65ded",
  "contract": "0x6ce2C11f53f1199685770BbFd2cCba5245D36B1b",
  "abi": [
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
}
```
