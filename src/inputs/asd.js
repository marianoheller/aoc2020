const fs = require("fs");
const data = fs.readFileSync("day_12.txt", "utf8").split("\n");
let x = 0,
    y = 0,
    dir = 0,
    part2 = false,
    ws = [10, 1];
for (i in data) {
    let direction = data[i].match(/\D/)[0];
    let steps = Number(data[i].match(/\d+\b/)[0]);
    switch (direction) {
        case "F":
            if (!part2) {
                if (dir == 0) x += steps
                if (dir == 1) y -= steps
                if (dir == 2) x -= steps
                if (dir == 3) y += steps
            } else {
                x += ws[0] * steps
                y += ws[1] * steps
            }
            break;
        case "R":
            dir += (steps / 90)
            dir = dir % 4
            if (part2) {
                for (let i = 0; i < (steps / 90); i++) {
                    ws = ws.reverse()
                    ws[1] = -1 * ws[1]
                }
            }
            break;
        case "L":
            dir += 4 - (steps / 90)
            dir = dir % 4
            if (part2) {
                for (let i = 0; i < (steps / 90); i++) {
                    ws = ws.reverse()
                    ws[0] = -1 * ws[0]
                }
            }
            break;
        case "W":
            if (!part2) x -= steps
            else ws[0] = ws[0] - steps
            break;
        case "E":
            if (!part2) x += steps
            else ws[0] = ws[0] + steps
            break;
        case "S":
            if (!part2) y -= steps
            else ws[1] = ws[1] - steps
            break;
        case "N":
            if (!part2) y += steps
            else ws[1] = ws[1] + steps
            break;
    }
}
console.log(Math.abs(x) + Math.abs(y))