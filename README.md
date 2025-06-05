# Esperanto To JS
エスペラントベースのプログラミング言語(笑)。
対格が引数、動詞が関数呼び出し。
```
Hogi estas adicii 1 kaj tion.
Numero estas hogi adicias 1 kaj 2.
Mi multiplikas 3 kaj numeron
```
コンパイル後（JS）
```js
function hog(ti) { return 1+ti };
let numer = hog(1+2);
3*numer;
```
