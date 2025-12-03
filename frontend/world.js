class World {
    chunkSize;
    tileSize;

    /**
     * @typedef {Object} Chunk
     * @property {(number | null)[][]} layers
     */

    /**
     * @type {Object.<number, Object.<number, Chunk>>}
     */
    chunks = {};

    /**
     * @typedef {Object} GameObject
     * @property {number} x
     * @property {number} y
     */

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

        this.objects[id] = {
            x, y,
        };
        return index;
    }

    get(x, y) {
        const row = this.chunks[y];
        if (!row) return undefined;
        return row[x];
    }
}

let world = new World();



/**
 * @type {GameObject}
 */
let player;
let player_id;
