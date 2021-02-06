import { Board } from "wasm-chess";

const board = Board.new();
// console.log(board.render());

document.addEventListener('click', (event) => {
    if (event.target.id) {
        console.log(event.target.id);
    } else if (event.target.parentNode.id) {
        console.log(event.target.parentNode.id);
    }
});
