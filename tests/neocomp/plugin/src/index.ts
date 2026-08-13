import { Context, render_list_enumerated, show_if } from '@neocomp/core';

let ctx = new Context(document.querySelector('#main')!);
let build = ctx.root_chunk();
let { html, signal, effect, computed } = build;
let count = signal(0);
let input = signal('');
let list = computed(() => input.value.split(','));
let log = signal(false);
html` <div class="test" class:active=${true} style:background="${'red'}">hallo world</div> `;
html`<input on:input=${(evt: any) => (input.value = evt.target.value)} />`;
html`
	<button on:click="${() => count.value++}">count: ${count}</button>
	<button on:click="${() => (log.value = !log.value)}">
		${() => (log.value ? 'log off' : 'log on')}
	</button>
	<div prop:innerHTML=${() => input.value}></div>
	<div style:color="${() => (log.value ? 'red' : 'green')}">
		before
		<${show_if(() => input.value !== 'hide')}>
		<${() => {
			build.cur_el.innerHTML = 'hello';
			html`${() => {
				let el = document.createElement('div');
				el.innerText = count.value.toString();
				return el;
			}}`;
			for (let i = 0; i < 10; i++) html`<div>${i}</div>`;
			if (Math.random() > 0.5) html`<div>random</div>`;
		}}>
		after
		
		<div><${render_list_enumerated(list, null, 'div', (build, item, ind) => {
			build.html`${ind}: ${item}`;
		})}></div>
	</div>

	<${() => {
		let chunk = ctx.removable_chunk('div');
		let chunk2 = ctx.removable_chunk('div');
		let count = signal(0);
		chunk2.html`<button on:click=${() => count.value++}>${count}</button>`;
		chunk2.effect(() => {
			let value1 = input.value,
				value2 = count.value;
			if (log.peek()) console.log(value1, value2);
		});
		chunk.html`${chunk2}`;
		html`${chunk}`;
		effect(() => {
			if (input.value === 'explode') chunk.remove();
		});
	}}>
`;
effect(() => {
	let value = count.value;
	if (log.peek()) console.log(value);
});
Reflect.set(globalThis, 'ctx', ctx);
