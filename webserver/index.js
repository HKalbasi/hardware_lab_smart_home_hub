import { WebSocketServer } from "ws";

const server = new WebSocketServer({ port: 8080 });

const groupId = 'djaksjdklsad';

const groups = { [groupId]: { state: { hub: false }, members: [], hub: undefined } };

const sendStateToMembers = (g) => {
  g.members.forEach((x) => x.send(JSON.stringify(g.state)));
}

server.on('connection', (ws) => {
  console.log('A new client connected');

  ws.on('message', (message) => {
    console.log(`Received message: ${message}`);
    const m = JSON.parse(message);
    const currentGroup = groups[groupId];
    if (m.type === 'hello') {
      if (m.me === 'hub') {
        currentGroup.hub = ws;
        currentGroup.state = { ...m.state, hub: true };
        sendStateToMembers(currentGroup);
      } else {
        currentGroup.members.push(ws);
        ws.send(JSON.stringify(groups[groupId].state));
      }
    }
    if (m.type === 'command') {
      currentGroup.hub.send(message);
    }
    if (m.type === 'update') {
      currentGroup.state = { ...m.state, hub: true };
      sendStateToMembers(currentGroup);
    }
  });

  ws.on('close', () => {
    const currentGroup = groups[groupId];
    if (currentGroup.hub === ws) {
      currentGroup.hub = undefined;
      currentGroup.state = { ...currentGroup.state, hub: false };
      sendStateToMembers(currentGroup);
    }
    console.log('Connection closed');
  });
});
