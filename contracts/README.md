# PoSol Verifying Contract

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

## Proof Data

```rust
proof: {
  b: Fp256 "(28319FC47334E5452B7460FF479F79282BC1474958A1625248E34AF5A6DA1764)"
  t: Fp256 "(2902B082B9AA5203112B8D743F22718BC6921E5FD8CB68CD2335A8A28D18EC4E)"
  h1: Fp256 "(17ACAE83BD297D5E2E1697FD57BB30D35DB7F41314E8AF8FAE679EDE39672B66)"
  h2: Fp256 "(17D054BFFE2B63828C312B283CEA8DC0FD9ACB6FAF7F061963DC3FB1AEFE6CBB)"
  sNext: Fp256 "(11743A905A1963B413CF828D42C50D5D16D7C0A87F2B993B590E92734A198CD9)"
  zNext: Fp256 "(2469E6293CC96A0F11260C3DD5DAC7B0F52F6B3E9B756611F3B5DC6C59A3801F)"
  h1Next: Fp256 "(2C5716B7CED3203F648805C1F9CEE83B511100B49C0A9622B90BD3AAF7B7672E)"
  h2Next: Fp256 "(07B38A4128C2778ACC83A6B3C9AB7A91B8806CD028F0D111C78B00923443B870)"
  bCommit: GroupAffine(x=Fp256 "(1B7B21D8FC850EA38A47978120BFF1E4C0A3C092C3E32A51B64401993E9645FC)", y=Fp256 "(1A532DDDE11802F78BED7C10586CAEC0C3EF36CFEB34559C1F896319925C2D6C)")
  sCommit: GroupAffine(x=Fp256 "(2CE65AB355E1D3C793E5EFC07FB5C6FC75AFF7C24B140C58ABCF61CEA87181EC)", y=Fp256 "(1463FAED773E750923785926FEB2ECA7711B5063CA837BFA8B3C842B216A6B7D)")
  h1Commit: GroupAffine(x=Fp256 "(079FB83FDC8FD17BA426EA2C00E3691365B7EF5959B9DF40B4AE73E413DDF6A7)", y=Fp256 "(1EAC736966F74342C116A506BC1C08E4ECF1556296E6EF2B5747177C2E81A2F9)")
  h2Commit: GroupAffine(x=Fp256 "(0DB51B5CBD5ABDBAC6C5D92EBAA625316F1FF401847EF390FF343AF193926EED)", y=Fp256 "(2D96EE2710E7CFC1E0E7C9AEF3E6AA89E3416D6A3C25BA06D8315C2A1CCCFCE2)")
  zCommit: GroupAffine(x=Fp256 "(08FB8D334C7B87D467F5B07C466248A5EA5C3C025A386037B27F6C8BB59AF531)", y=Fp256 "(13AEF4DBBD54AD73DB005731E169A49CFCCC03A30718778B4692F62904D88A49)")
  q1Commit: GroupAffine(x=Fp256 "(21D581F38B6910B54CB499D0F1B5FF95858D2E015235AFFCD8F5F0EEFAF1C437)", y=Fp256 "(02A8B0C4058AB5E0309DB277106B4D091825388CF03608CF65F921FEA8937D96)")
  q2Commit: GroupAffine(x=Fp256 "(2433F05F83CEB701C7B6D4152FD3C59F6470020F7F39183ACA40B6B276EEADF4)", y=Fp256 "(0C0CC540A47B5D227FEFE7DC8D5EDB4104F716A41393D8DF04A3438D5AF2BC0A)")
  opening1: GroupAffine(x=Fp256 "(0519A1B094706C33F7A7F4DC60E1643025E1ADFB66D1AD61035A632180875392)", y=Fp256 "(1092E5CFFD177431128BD5AE17FD99A11E8279DD4AE46CB0E294082D38BE7984)")
  opening2: GroupAffine(x=Fp256 "(03AEBCAC21653CD9A67BE0A4CF89E34D6BD280F092E6086EB115F685C65744AE)", y=Fp256 "(2CA6FD563BD039D8D412DD884CDA9D9F3E226847B7374774A0E40EDF501BCAE6)")
}
balance sum: Fp256 "(0000000000000000000000000000000000000000000000000000000075C7DC8E)"
```
