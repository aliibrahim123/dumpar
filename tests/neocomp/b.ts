import { Context } from '../src/index.ts';

let ctx = new Context(document.createElement('div'));
let build = ctx.root_chunk();
let count = build.signal(0);
build.html`<button on:click=${() => count.value++}>count: ${count}</button>`;
