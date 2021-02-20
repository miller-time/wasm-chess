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

  if (debug) {
    console.log('clicked on:', square);
  }
  if (piece) {
    if (debug) {
      console.log('piece:', piece.innerHTML);
    }
  }
  if (heldPiece || heldPieceLocation) {
    if (debug) {
      console.log('holding:', heldPiece.innerHTML, 'located:', heldPieceLocation);
    }
  }

  if (piece) {
    // this square has a piece on it
    if (heldPieceLocation) {
      // we were holding a piece
      if (heldPieceLocation === square) {
        // put the held piece back down
        console.log('returning', heldPiece.innerHTML, 'to', heldPieceLocation);
        heldPiece.classList = 'piece';
        heldPiece = null;
        heldPieceLocation = null;
      } else {
        // capture this piece
        capturePiece(heldPieceLocation, square);
        // reset
        heldPiece.classList = 'piece';
        heldPiece = null;
        heldPieceLocation = null;
      }
    } else {
      // pick up this piece to move it
      piece.classList = 'piece held';
      heldPiece = piece;
      heldPieceLocation = square;
      console.log('picking up', heldPiece.innerHTML, 'from', heldPieceLocation);
    }
  } else if (heldPieceLocation) {
    // we were holding a piece, move it to this square
    movePiece(heldPieceLocation, square);
    // reset
    heldPiece.classList = 'piece';
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
      console.log('moving piece', fromPiece.innerHTML, 'from', from, 'to', to);
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

function capturePiece(captor, captive) {
  const captorSquare = document.getElementById(captor);
  if (debug) {
    console.log('captorSquare', captorSquare);
  }
  const captiveSquare = document.getElementById(captive);
  if (debug) {
    console.log('captiveSquare', captiveSquare);
  }
  if (captorSquare && captiveSquare) {
    const captorPiece = captorSquare.querySelector('.piece');
    if (debug) {
      console.log('captorPiece', captorPiece);
    }
    const captivePiece = captiveSquare.querySelector('.piece');
    if (debug) {
      console.log('captivePiece', captivePiece);
    }
    if (captorPiece && captivePiece) {
      console.log(
        'moving', captorPiece.innerHTML, 'from', captor, 'to', captive,
        'and capturing', captivePiece.innerHTML
      );
      captivePiece.remove();
      captiveSquare.append(captorPiece);

      // this is a spacer element that unifies the height of empty squares
      // to match the height of squares with pieces
      const empty = document.createElement('span');
      empty.classList = 'empty';
      empty.innerHTML = '<br />';
      captorSquare.append(empty);
    }
  }
}
