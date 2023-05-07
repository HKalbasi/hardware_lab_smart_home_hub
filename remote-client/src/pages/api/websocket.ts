

let ws: WebSocket | undefined;

const send = (data: any) => {
  ws?.send(JSON.stringify(data));
}

let subscribers: any[] = [];

export const sendCommand = (command: any) => {
  send({ ...command, type: 'command' });
};

export const subscribe = (f: any) => {
  if (!ws) {
    ws = new WebSocket('ws://127.0.0.1:8080');
    ws.onopen = () => {
      send({ type: 'hello', me: 'client' });
    };
    ws.onmessage = (msg: MessageEvent) => {
      const data = JSON.parse(msg.data);
      subscribers.forEach((f) => f(data));
    };
    ws.onclose = () => {
      subscribers.forEach((f) => f(undefined));
      setTimeout(() => window.location.reload(), 1000);
    };
  }
  subscribers.push(f);
  return () => {
    subscribers = subscribers.filter((x) => x !== f)
  };
};
