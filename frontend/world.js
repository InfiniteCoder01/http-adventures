function parseString(bytes, index) {
  let end = index;
  while (bytes.getUint8(end) != 0) end++;
  return [new TextDecoder("utf-8").decode(bytes.buffer.slice(index, end)), end + 1];
}

class GameObject {
    x; y;
    texture;
    facing = 'south';

    constructor(x, y, texture) {
        this.x = x, this.y = y;
        this.texture = texture;
    }

    path = [];
    pathfind(x, y) {
        function* dirs(priority) {
            const dirs = { 'north': [0, -1], 'east': [1, 0], 'south': [0, 1], 'west': [-1, 0] };
            if (priority) yield [priority, dirs[priority]];
            for (const [dir, v] of Object.entries(dirs)) {
                if (dir == priority) continue;
                yield [dir, v];
            }
        }

        const start = [Math.floor(this.x), Math.floor(this.y)];
        const queue = [{
            path: this.path.length > 0 ? [start, this.path[0]] : [start],
            face: this.facing,
        }];
        if (this.path.length > 0) this.path.splice(1);

        const visited = {};
        for (const v of queue[0].path) visited[v] = true;

        while (queue.length > 0) {
            const { path, face } = queue.shift();

            for (const [dir, v] of dirs(face)) {
                const next = [
                    path[path.length - 1][0] + v[0],
                    path[path.length - 1][1] + v[1]
                ];

                if (visited[next]) continue;
                visited[next] = true;
                const newPath = [...path, next];

                if (next[0] == x && next[1] == y) {
                    newPath.shift();
                    this.path = newPath;
                    return;
                } else if (newPath.length > 15) continue;

                queue.push({
                    path: newPath,
                    face: dir,
                });
            }
        }
    }

    update() {
        if (this.path.length > 0) {
            const [x, y] = this.path[0];
            const speed = 1 / 16;
            if (this.x < x) this.x += speed, this.facing = 'east';
            else if (this.x > x) this.x -= speed, this.facing = 'west';
            else if (this.y < y) this.y += speed, this.facing = 'south';
            else if (this.y > y) this.y -= speed, this.facing = 'north';
            else this.path.shift();
        }
    }

    size() {
        return [2, 5];
    }
}

class World {
    chunkSize;
    tileset;
    tileSize;
    offsets = [];

    /**
     * @typedef {Object} Chunk
     * @property {(number | null)[][]} layers
     */

    /**
     * @type {Object.<number, Object.<number, Chunk>>}
     */
    chunks = {};

    /**
     * @type {Object.<number, GameObject>}
     */
    objects = {};

    /**
     * @param bytes {DataView}
     */
    parseChunk(bytes, index) {
        const layers = bytes.getUint8(index++);
        const x = bytes.getUint32(index);
        const y = bytes.getUint32(index + 4);
        index += 8;

        const chunk = {
            layers: [],
        };
        for (let layer = 0; layer < layers; layer++) {
            const layer = [];
            for (let i = 0; i < this.chunkSize * this.chunkSize; i++) {
                const tile = bytes.getUint32(index);
                index += 4;
                if (tile === 0xffffffff) layer.push(null);
                else layer.push(tile);
            }
            chunk.layers.push(layer);
        }

        if (!this.chunks.hasOwnProperty(y)) this.chunks[y] = {}
        this.chunks[y][x] = chunk;
        return index;
    }

    /**
     * @param bytes {DataView}
     */
    parseObject(bytes, index) {
        const change = String.fromCharCode(bytes.getUint8(index++));
        const id = bytes.getUint32(index);
        index += 4;
        if (change == '-') {
            delete this.objects[id];
            return index;
        }

        const x = bytes.getUint32(index);
        const y = bytes.getUint32(index + 4);
        index += 8;
        let texture;
        [texture, index] = parseString(bytes, index);

        this.objects[id] = new GameObject(x, y, texture);
        return index;
    }

    get(x, y) {
        const row = this.chunks[y];
        if (!row) return undefined;
        return row[x];
    }

    update() {
        for (const object of Object.values(this.objects)) {
            object.update();
        }
    }
}

let world = new World();

/**
 * @type {GameObject}
 */
let player;
let player_id;
