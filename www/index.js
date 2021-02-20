import { Board } from "wasm-chess";

// set to true for verbose logging
const debug = false;

let heldPiece;
let heldPieceLocation;

const board = Board.new();
// console.log(board.render());

document.addEventListener('click', (event) => {
  if (debug) {
    console.log('event.target', event.target);
  }
  let piece;
  let square;
  if (event.target.id) {
    square = event.target.id;
    piece = event.target.querySelector('.piece');
  } else if (event.target.parentNode.id) {
    square = event.target.parentNode.id;
    piece = event.target.parentNode.querySelector('.piece');
  } else {
    square = null;
    piece = null;
  }

  console.log('clicked on:', square);
  if (piece) {
    if (debug) {
      console.log('piece:', piece.innerHTML);
    }
  }
  if (heldPiece || heldPieceLocation) {
    if (debug) {
      console.log('holding:', heldPiece, 'located:', heldPieceLocation);
    }
  }

  if (piece) {
    // this square has a piece on it
    if (heldPieceLocation) {
      // we were already holding a piece, cancel that
      console.log('bumped into', piece.innerHTML, 'at', square);
      console.log('returning', heldPiece, 'to', heldPieceLocation);
      heldPiece = null;
      heldPieceLocation = null;
    } else {
      // pick up this piece to move it
      heldPiece = piece.innerHTML;
      heldPieceLocation = square;
      console.log('picking up', piece.innerHTML, 'from', heldPieceLocation);
    }
  } else if (heldPieceLocation) {
    // we were holding a piece, move it to this square
    movePiece(heldPieceLocation, square);
    // reset
    heldPiece = null;
    heldPieceLocation = null;
  }
  if (debug) {
    console.log('***********');
  }
});

function movePiece(from, to) {
  const fromSquare = document.getElementById(from);
  if (debug) {
    console.log('fromSquare', fromSquare);
  }
  const toSquare = document.getElementById(to);
  if (debug) {
    console.log('toSquare', toSquare);
  }
  if (fromSquare && toSquare) {
    const fromPiece = fromSquare.querySelector('.piece');
    if (debug) {
      console.log('fromPiece', fromPiece);
    }
    if (fromPiece) {
      console.log('moving piece', heldPiece, 'from', from, 'to', to);
      toSquare.append(fromPiece);
    }
    // this is a spacer element that unifies the height of empty squares
    // to match the height of squares with pieces
    const toEmpty = toSquare.querySelector('.empty');
    if (toEmpty) {
      fromSquare.append(toEmpty);
    }
  }
}
