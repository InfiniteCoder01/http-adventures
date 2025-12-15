class GameObject {
    x; y;
    texture;
    facing = 'south';
    target = null;
    moveCallback = _ => undefined;
    reachCallback = _ => undefined;

    constructor(x, y, tex) {
        this.x = x, this.y = y;
        this.texture = texture(tex);
    }

    path = [];
    pathfind(x, y) {
        function* dirs(priority) {
            const dirs = { 'south': [0, 1], 'east': [1, 0], 'west': [-1, 0], 'north': [0, -1] };
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

        const visited = {};
        for (const v of queue[0].path) visited[v] = true;

        while (queue.length > 0) {
            const { path, face } = queue.shift();
            if (path[path.length - 1][0] == x && path[path.length - 1][1] == y) {
                this.path = path.splice(1);
                return;
            } else if (path.length >= 15) continue;


            for (const [dir, v] of dirs(face)) {
                const next = [
                    path[path.length - 1][0] + v[0],
                    path[path.length - 1][1] + v[1]
                ];

                if (visited[next]) continue;
                visited[next] = true;
                queue.push({
                    path: [...path, next],
                    face: dir,
                });
            }
        }
    }

    lastMoving = false;
    update() {
        if (Array.isArray(this.target)) {
            this.pathfind(...this.target);
        } else if (this.target) {
            const target = world.obj(this.target);
            if (target) this.pathfind(Math.floor(target.x), Math.floor(target.y));
        }

        if (this.path.length > 0) {
            if (!this.lastMoving) this.moveCallback(this.path[0]);
            this.lastMoving = true;

            const [x, y] = this.path[0];
            const speed = 1 / 16;
            if (this.x < x) this.x += speed, this.facing = 'east';
            else if (this.x > x) this.x -= speed, this.facing = 'west';
            else if (this.y < y) this.y += speed, this.facing = 'south';
            else if (this.y > y) this.y -= speed, this.facing = 'north';
            else {
                this.path.shift();
                if (this.path.length > 0) this.moveCallback(this.path[0]);
            }
        } else if (this.target) {
            this.lastMoving = false;
            this.reachCallback(this.target);
            this.target = null;
        }
    }

    size() {
        return [2, 5];
    }
}

