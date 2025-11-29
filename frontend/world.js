class World {
    /**
     * @typedef {Object} Chunk
     * @property {(number | null)[][]} layers
     */

    chunkSize;
    tileSize;

    /**
     * @type {Object.<number, Object.<number, Chunk>>}
     */
    chunks = {};

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

    get(x, y) {
        const row = this.chunks[y];
        if (!row) return undefined;
        return row[x];
    }
}

let world = new World();
