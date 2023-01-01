// test

declare namespace Console {
  interface Console {
    log(text: string): void;
  }

  var console: Console;
}
