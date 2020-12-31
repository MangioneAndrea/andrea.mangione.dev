import * as PIXI_TYPE from "pixi.js";
import { getWidth } from "../../CommonFunctions";
import * as Colours from "../Colours";

class RainDrop {
    /**
     * 
     * @param {PIXI_TYPE.Graphics} graphics 
     */
    constructor(graphics, { onDestroy, containerWidth, containerHeight }) {
        this.circleRadius = 35;
        this.color = Colours.getRandomColour();
        this.ticksToExplode = 75
        this.lifetime = Math.random() * 420 + 30;
        this.graphics = graphics;
        this.containerHeight = containerHeight;
        this.x = Math.clamp(Math.random() * containerWidth, this.circleRadius, containerWidth - this.circleRadius);
        this.y = 0;
        this.height = 50;
        this.width = 10;
        this.next();
        this.destroy = () => onDestroy(this);
    }

    next() {
        this.lifetime--;
        if (this.lifetime < 0) {
            if (this.lifetime < -this.ticksToExplode) {
                this.destroy();
            } else {
                const stage = this.lifetime / -this.ticksToExplode;
                this.graphics.lineStyle(5, this.color, 1 - stage);
                this.graphics.drawCircle(this.x + this.width / 2, this.y + this.height, stage * this.circleRadius);
            }
        } else {
            this.y += 3;
            this.graphics.lineStyle(1, this.color);
            this.graphics.beginFill(this.color);
            this.graphics.drawRoundedRect(this.x, this.y, this.width, this.height, 5);
            this.graphics.endFill();
        }
        if (this.containerHeight < this.y) this.destroy();
    }
}

export default RainDrop