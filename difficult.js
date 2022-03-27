const calDifficulty = ({distance, up, down}) => Math.round(distance / ((up + down) / distance));

const hiuchi = {
    distance: 16_300,
    up: 946,
    down: 945,
}

const takao = {
    distance: 4_000,
    up: 515,
    down: 108
}

const hiuchiDiff = calDifficulty(hiuchi);
const takaoDiff = calDifficulty(takao)

console.log("hiuchi", hiuchiDiff)
console.log("takao", takaoDiff)
console.log(hiuchiDiff / takaoDiff)