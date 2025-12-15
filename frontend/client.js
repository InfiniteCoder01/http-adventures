const url = `${document.location.origin.replace('http', 'ws')}/api`;
const socket = new WebSocket(url);

function onUpdateMessage(bytes, join) {
    let index = 1;

    if (join) { // Parse join metadata
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

    // Parse chunks and objects
    while (bytes.getUint8(index) > 0) index = world.parseChunk(bytes, index);
    index++;
    while (bytes.getUint8(index) > 0) index = world.parseObject(bytes, index);
    index++;

    if (join) { // Get the player and setup callbacks
      player = bytes.getUint32(index);
      const plr = world.obj(player);
      plr.moveCallback = cell => socket.send(blob('u', uints(...cell)));
      plr.reachCallback = target => {
        if (Array.isArray(target)) return;
        socket.send(blob('i', uints(target)))
      }
    }
}

// On message
socket.addEventListener("message", async (event) => {
  const bytes = new DataView(await event.data.arrayBuffer());
  const type = String.fromCharCode(bytes.getUint8(0));

  if (type == 'j' || type == 'u') onUpdateMessage(bytes, type == 'j');
  else console.error(`Unknown message type ${type} (${bytes.getUint8(0)})`);
});

// Sending data
const blob = (...parts) => new Blob(parts);
const uints = (...numbers) => {
  const view = new DataView(new ArrayBuffer(numbers.length * 4));
  for (const i in numbers) view.setUint32(i * 4, numbers[i]);
  return view;
};

