import { WebSocketServer } from 'ws';
import { createServer } from 'https';
import { readFileSync } from 'fs';

/*
const server = createServer({
    cert: readFileSync('cert.pem'),
    key: readFileSync('key.pem')
});
*/
const wss = new WebSocketServer({ port: 8080 });

wss.on('connection', function connection(ws: any) {
    console.log("conn");
    ws.on('error', console.error);

    ws.on('message', function message(data: any) {
        let sdata = data.toString("hex").match(/../g).join(" ").split(" 00 51 ");

        console.log(sdata[0] + " 00");
        console.log();
        //console.log(sdata[1].slice(0, 20));
        console.log("51 00 " + sdata[1]);
        console.log("=================================================================");
    });

    ws.send('something');
});

/*
server.listen(8443);
*/
