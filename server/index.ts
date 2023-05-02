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
        console.log(data.toString("hex"));
    });

    ws.send('something');
});

/*
server.listen(8443);
*/
