const url = `${document.location.origin.replace('http', 'ws')}/api`;
const socket = new WebSocket(url);

function parseString(bytes, index) {
  let end = index;
  while (bytes.getUint8(end) != 0) end++;
  return [new TextDecoder("utf-8").decode(bytes.buffer.slice(index, end)), end + 1];
}

// On message
socket.addEventListener("message", async (event) => {
  const bytes = new DataView(await event.data.arrayBuffer());
  const type = String.fromCharCode(bytes.getUint8(0));

  if (type == 't') { // Texture message
    const [id, index] = parseString(bytes, 1);
    textures[id] = await createImageBitmap(event.data.slice(index));
  } else if (type == 'j' || type == 'u') { // Join/update message
    let index = 1;

    if (type == 'j') { // Parse join metadata
      world = new World();
      world.chunkSize = bytes.getUint32(index);
      world.tileSize = bytes.getUint32(index + 4);
      index += 8;
    }

    while (bytes.getUint8(index) > 0) index = world.parseChunk(bytes, index);
    index++;
  } else { // Unknown message
    console.error(`Unknown message type ${type} (${bytes.getUint8(0)})`);
  }
});

// Sending data
const blob = (...parts) => new Blob(parts);
const floats = (...numbers) => {
  const view = new DataView(new ArrayBuffer(numbers.length * 4));
  for (const i in numbers) view.setFloat32(i * 4, numbers[i]);
  return view;
};

function sendPlayerUpdate() {
  // socket.send(blob('u', floats(camera.x, camera.y)));
}

