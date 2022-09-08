#!/usr/bin/env node
import express from 'express';
import http from 'http';
import path from 'path';

const normalizePort = (val: string) => {
  const port = parseInt(val, 10);
  if (isNaN(port)) {
    return val;
  }
  if (port >= 0) {
    return port;
  }
  return false;
};

const onError = (error: any) => {
  if (error.syscall !== 'listen') {
    throw error;
  }
  const bind = typeof port === 'string' ? 'Pipe ' + port : 'Port ' + port;
  switch (error.code) {
    case 'EACCES':
      console.error(bind + ' requires elevated privileges');
      process.exit(1);
      break;
    case 'EADDRINUSE':
      console.error(bind + ' is already in use');
      process.exit(1);
      break;
    default:
      throw error;
  }
};

const onListening = () => {
  const addr = server.address();
  const bind = typeof addr === 'string' ? 'pipe ' + addr : 'port ' + addr?.port;
  console.log('Listening on ' + bind);
};

const app = express();
const port = normalizePort(process.env.PORT || '3000');
app.set('port', port);
app.use(express.static(path.join(__dirname, 'public')));
app.get('/', (req, res) => {
  const isJA = req.acceptsLanguages(['ja']);
  const filename = isJA ? 'ja' : 'en';
  res.sendFile(path.join(__dirname, `public/${filename}.html`));
});
const server = http.createServer(app);
server.listen(port);
server.on('error', onError);
server.on('listening', onListening);
