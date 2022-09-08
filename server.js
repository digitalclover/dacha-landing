#!/usr/bin/env node
var express = require('express');
var http = require('http');
var path = require('path');
var normalizePort = function (val) {
    var port = parseInt(val, 10);
    if (isNaN(port)) {
        return val;
    }
    if (port >= 0) {
        return port;
    }
    return false;
};
var onError = function (error) {
    if (error.syscall !== 'listen') {
        throw error;
    }
    var bind = typeof port === 'string' ? 'Pipe ' + port : 'Port ' + port;
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
var onListening = function () {
    var addr = server.address();
    var bind = typeof addr === 'string' ? 'pipe ' + addr : 'port ' + (addr === null || addr === void 0 ? void 0 : addr.port);
    console.log('Listening on ' + bind);
};
var app = express();
var port = normalizePort(process.env.PORT || '3000');
app.set('port', port);
app.use(express.static(path.join(__dirname, 'public')));
app.get('/', function (req, res) {
    var isJA = req.acceptsLanguages(['ja']);
    var filename = isJA ? 'ja' : 'en';
    res.sendFile(path.join(__dirname, "public/".concat(filename, ".html")));
});
var server = http.createServer(app);
server.listen(port);
server.on('error', onError);
server.on('listening', onListening);
