# NeonDB research

## Login (First query)
`00 00 00 3c 00 03 00 00 75 73 65 72 00 66 69 6c 69 70 74 6f 6e 00 64 61 74 61 62 61 73 65 00 6e 65 6f 6e 64 62 00 63 6c 69 65 6e 74 5f 65 6e 63 6f 64 69 6e 67 00 55 54 46 38 00 00 70 00 00 00 11 6b 61 43 66 64 79 50 34 65 55 33 6c 00`

- **00 00 00 3C 00 03 00 00** - characters at start?? 
- **75 73 65 72** - "user"
- **00** - blank
- **66 69 6c 69 70 74 6f 6e** - "filipton" (username)
- **00** - blank 
- **64 61 74 61 62 61 73 65** - "database"
- **00** - blank 
- **6e 65 6f 6e 64 62** - "neondb" (database) 
- **00** - blank
- **63 6c 69 65 6e 74 5f 65 6e 63 6f 64 69 6e 67** - "client_encoding" 
- **00** - blank 
- **55 54 46 38** - "UTF8" (encoding) 
- **00 00** - 2 x blank
- **70** - p 
- **00 00 00 11** - passowrd length + 5 (?)
- **6b 61 43 66 64 79 50 34 65 55 33 6c** - "9l2FcxtusEYC" (password) 
- **00** - blank

## Query:
`51 00 00 00  05  00` - "" <br />
`51 00 00 00  06  31 00` - "1" <br />
`51 00 00 00  07  31 32 00` - "12" <br />
`51 00 00 00  08  31 32 33 00` - "123" <br />
`51 00 00 00  0f  31 32 33 34 35 36 37 38 39 30 00` - "1234567890" <br />

- **51** - Q
- **00 00 00 06** - query length  + 5(?)
- **31** - query ...
- **00** - one blank at the end

[PROTOCOL LINK](https://github.com/brianc/node-postgres/blob/master/packages/pg-protocol/src/parser.ts#L212)
