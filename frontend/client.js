const url = `${document.location.origin.replace('http', 'ws')}/api`;
const socket = new WebSocket(url);

socket.addEventListener("open", () => {
  socket.send("Hello Server!");
});

socket.addEventListener("message", async (event) => {
  const bytes = new Uint8Array(await event.data.arrayBuffer());
  const type = String.fromCharCode(bytes[0]);

  if (type == 't') {
    const sep = bytes.indexOf(0);
    const id = await event.data.slice(1, sep).text();
    textures[id] = await createImageBitmap(event.data.slice(sep + 1));
    console.log(textures);
    ctx.drawImage(textures['test.png'], 16, 16, 128, 128);
  }
});

