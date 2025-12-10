const url = `${document.location.origin.replace('http', 'ws')}/api`;
const socket = new WebSocket(url);

// On message
socket.addEventListener("message", async (event) => {
  const bytes = new DataView(await event.data.arrayBuffer());
  const type = String.fromCharCode(bytes.getUint8(0));

  if (type == 'j' || type == 'u') { // Join/update message
    let index = 1;

    if (type == 'j') { // Parse join metadata
      world = new World();
      world.chunkSize = bytes.getUint32(index);
      [world.tileset, index] = parseString(bytes, index + 4);
      world.tileset = texture(world.tileset);
      world.tileSize = bytes.getUint32(index);
      const nlayers = bytes.getUint32(index + 4);
      index += 8;
      for (let i = 0; i < nlayers; i++) {
        world.offsets.push(bytes.getUint32(index));
        index += 4;
      }
    }

    while (bytes.getUint8(index) > 0) index = world.parseChunk(bytes, index);
    index++;
    while (bytes.getUint8(index) > 0) index = world.parseObject(bytes, index);
    index++;

    if (type == 'j') {
      player_id = bytes.getUint32(index)
      player = world.objects[player_id];
    }
  } else { // Unknown message
    console.error(`Unknown message type ${type} (${bytes.getUint8(0)})`);
  }
});

// Sending data
const blob = (...parts) => new Blob(parts);
const uints = (...numbers) => {
  const view = new DataView(new ArrayBuffer(numbers.length * 4));
  for (const i in numbers) view.setUint32(i * 4, numbers[i]);
  return view;
};

function sendPlayerUpdate() {
  socket.send(blob('u', uints(player.x, player.y)));
}

