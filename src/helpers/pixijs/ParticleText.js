
import * as PIXI from "pixi.js";
import opentype from 'opentype.js'
class Particles {
    constructor(x, y, texture, size) {
        this.x = x;
        this.y = y;

        this.sprite = new PIXI.Sprite(new PIXI.Texture.from(texture));

        this.sprite.texture.frame = new PIXI.Rectangle(x, y, size, size);

        this.sprite.x = x;
        this.sprite.y = y;

        this.speedX = 0;
        this.speedY = 0;

        this.radius = 50;

        this.gravity = 0.1;

        this.maxGravity = 0.01 + Math.random() * 0.03;

        this.friction = 0.8;
    }

    update(mouse, fps) {
        // Make the frames indipendent as the latence is calculated and skipped
        const framePivot = fps / 16.66
        const distanceX = mouse.x - this.sprite.x;
        const distanceY = mouse.y - this.sprite.y;

        const distanceSqrd = distanceX * distanceX + distanceY * distanceY;
        if (distanceSqrd < this.radius * this.radius && distanceSqrd > 0) {
            const distance = Math.sqrt(distanceSqrd);

            let normalX = distanceX / distance;
            let normalY = distanceY / distance;

            this.speedX -= normalX;
            this.speedY -= normalY;

            this.gravity *= this.friction;
        } else {
            this.gravity += 0.1 * (this.maxGravity - this.gravity);
        }

        let oDistX = this.x - this.sprite.x;
        let oDistY = this.y - this.sprite.y;

        this.speedX += oDistX * this.gravity;
        this.speedY += oDistY * this.gravity;

        this.speedX *= this.friction;
        this.speedY *= this.friction;

        this.sprite.x += this.speedX * framePivot;
        this.sprite.y += this.speedY * framePivot;
    }
}
class ParticlesText {
    constructor({ pixiApp, text, size }) {
        this.app = pixiApp;

        this.text = text;
        this.size = size;

        this.cols = 500;
        this.rows = 200;

        this.pSize = 1;
        this.particles = [];

        this.mouse = { x: 0, y: 0 }
        this.container = new PIXI.ParticleContainer(5000);
        this.container.setTransform(0, 0);
        this.app.stage.addChild(this.container);

    }

    init() {
        const that = this;
        opentype.load(
            "./font.ttf", //"https://raw.githack.com/AlainBarrios/Fonts/master/LeagueSpartan-Bold.otf",
            function (err, font) {
                if (err) {
                    console.warn("Font could not be loaded: " + err);
                } else {
                    const canvas = document.createElement("canvas");

                    // Now let's display it on a canvas with id "canvas"
                    const ctx = canvas.getContext("2d");

                    // Construct a Path object containing the letter shapes of the given text.
                    // The other parameters are x, y and fontSize.
                    // Note that y is the position of the baseline.
                    const path = font.getPath(that.text, 0, that.size, that.size);
                    const width = font.getAdvanceWidth(that.text, that.size);


                    that.cols = width;
                    that.rows = that.size;

                    canvas.width = width;
                    canvas.height = that.size;

                    path.fill = "rgb(0,0,0)";
                    ctx.fillStyle = "#000000";
                    ctx.imageSmoothingEnabled = false;
                    // If you just want to draw the text you can also use font.draw(ctx, text, x, y, fontSize).
                    path.draw(ctx);

                    that.addObjects(canvas, ctx);
                }
            }
        );
    }

    isEmpty(x, y, ctx) {
        const image = ctx.getImageData(x, y, this.pSize, this.pSize);

        let emptyCounter = 0;

        for (let i = 0; (length = image.data.length), i < length; i += 4) {
            if (image.data[i + 3] !== 0) {
                continue;
            }
            emptyCounter++;
        }

        return emptyCounter === this.pSize * this.pSize;
    }

    addObjects(canvas, ctx) {
        this.container.x = this.app.renderer.width / 2 - this.cols / 2;
        this.container.y = this.app.renderer.height / 2 - this.rows / 2;

        for (let i = 0; i < this.cols; i += 1) {
            for (let j = 0; j < this.rows; j += 1) {
                const x = i * this.pSize;
                const y = j * this.pSize;

                if (this.isEmpty(x, y, ctx)) continue;

                const p = new Particles(x, y, canvas, this.pSize);

                this.particles.push(p);
                this.container.addChild(p.sprite);
            }
        }

        this.container.interactive = true;

        this.container.on("mousemove", e => {
            this.mouse = e.data.getLocalPosition(this.container);
        });

        this.animate();
        this.app.ticker.maxFPS = 60
    }


    animate() {
        this.app.ticker.add(() => {
            this.particles.forEach(p => {
                p.update(this.mouse, this.app.ticker.deltaMS);
            });
        });
    }
}

export const createText = (pixiApp, text, props) => {
    const particles = new ParticlesText({ pixiApp, text, ...props });
    particles.init();
}