let drawDebugInfo = false;
const camera = {
    x: 0,
    y: 0,
};

function getTransforms() {
    const minSize = 400;
    const scale = Math.round(Math.min(canvas.width, canvas.height) / minSize);
    const width = canvas.width / scale, height = canvas.height / scale;
    return [
        scale,
        Math.round(-camera.x + width / 2), Math.round(-camera.y + height / 2),
        width, height,
    ];
}

function cameraToWorld(x, y) {
    const [scale, dx, dy, _width, _height] = getTransforms();
    return [(x / scale - dx) / world.tileSize, (y / scale - dy) / world.tileSize];
}

const textures = {};
function texture(id) {
    if (!textures[id]) {
        textures[id] = new Image();
        textures[id].src = `${document.location.origin}/assets/${id}`;
    }
    return textures[id];
}

function drawTile(image, tile, tileSize, x, y) {
    if (typeof tileSize === "number") tileSize = [tileSize, tileSize];
    if (typeof image === "string") image = texture(image);
    ctx.drawImage(
        image,
        tile[0] * tileSize[0],
        tile[1] * tileSize[1],
        ...tileSize, x, y, ...tileSize);
}

function drawChunk(cx, cy, layerIndex) {
    const chunk = world.get(cx, cy);
    if (!chunk) return;
    const layer = chunk.layers[layerIndex];
    for (let tx = 0; tx < world.chunkSize; tx++) {
        for (let ty = 0; ty < world.chunkSize; ty++) {
            const tile = layer[tx + ty * world.chunkSize];
            if (tile === null) continue;
            const row = world.tileset.width / world.tileSize;
            drawTile(
                world.tileset,
                [tile % row, Math.floor(tile / row)],
                world.tileSize,
                (cx * world.chunkSize + tx) * world.tileSize + world.offsets[layerIndex],
                (cy * world.chunkSize + ty) * world.tileSize + world.offsets[layerIndex],
            );
        }
    }

    if (drawDebugInfo) {
        ctx.strokeStyle = "red";
        ctx.strokeRect(
            cx * world.chunkSize * world.tileSize,
            cy * world.chunkSize * world.tileSize,
            world.chunkSize * world.tileSize,
            world.chunkSize * world.tileSize
        );
    }
}

function drawObject(object) {
    const image = texture(object.texture);
    ctx.drawImage(image, (object.x + 0.5) * world.tileSize - image.width / 2, (object.y + 0.5) * world.tileSize - image.height);
    if (drawDebugInfo) {
        ctx.strokeStyle = "red";
        const point = (x, y) => [(x + 0.5) * world.tileSize, (y + 0.5) * world.tileSize];
        ctx.beginPath();
        ctx.moveTo(...point(object.x, object.y));
        for (const [x, y] of object.path) {
            ctx.lineTo(...point(x, y));
        }
        ctx.stroke();
    }
}

function renderFrame() {
    const [scale, dx, dy, width, height] = getTransforms();
    ctx.resetTransform();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.scale(scale, scale);
    ctx.translate(dx, dy);

    const min = c => Math.floor(c / world.tileSize / world.chunkSize);
    const max = c => Math.ceil(c / world.tileSize / world.chunkSize);
    for (const layer in world.offsets) {
        for (let cy = min(camera.y - height / 2); cy < max(camera.y + height / 2); cy++) {
            for (let cx = min(camera.x - width / 2); cx < max(camera.x + width / 2); cx++) {
                drawChunk(cx, cy, layer);
            }
        }
    }

    const objects = Object.values(world.objects)
    objects.sort((a, b) => a.y - b.y);
    for (const object of objects) drawObject(object);

    requestAnimationFrame(renderFrame);
}

requestAnimationFrame(renderFrame);
