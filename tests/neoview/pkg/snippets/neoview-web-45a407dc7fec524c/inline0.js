const common_names = ["a","abbr","accept","action","address","align-content","align-items","align-self","alt","animation","area","aria-controls","aria-describedby","aria-expanded","aria-hidden","aria-label","article","aside","aspect-ratio","async","audio","autocomplete","autofocus","autoplay","b","backdrop-filter","background","background-attachment","background-color","background-image","background-position","background-repeat","background-size","base","bdi","bdo","blockquote","body","border","border-bottom","border-color","border-left","border-radius","border-right","border-style","border-top","border-width","box-shadow","box-sizing","br","button","canvas","caption","change","charset","checked","cite","class","clear","click","clip-path","code","col","colgroup","color","cols","colspan","column-gap","content","controls","crossorigin","cursor","data","datalist","dd","defer","del","details","dfn","dialog","dir","disabled","display","div","dl","download","dt","em","enctype","fieldset","figcaption","figure","filter","flex","flex-basis","flex-direction","flex-grow","flex-shrink","flex-wrap","float","focus","font-family","font-size","font-style","font-weight","footer","for","form","gap","grid","grid-area","grid-column","grid-row","grid-template-areas","grid-template-columns","grid-template-rows","h1","h2","h3","h4","h5","h6","head","header","headers","height","hidden","hr","href","html","http-equiv","i","id","iframe","img","input","ins","inset","integrity","justify-content","kbd","label","lang","left","legend","letter-spacing","li","line-height","link","list-style","list-style-type","loading","loop","main","map","margin","margin-bottom","margin-left","margin-right","margin-top","mark","max","max-height","max-width","maxlength","meta","meter","method","min","min-height","min-width","minlength","multiple","muted","name","nav","noscript","novalidate","ol","opacity","open","optgroup","option","order","outline","output","overflow","overflow-x","overflow-y","p","padding","padding-bottom","padding-left","padding-right","padding-top","pattern","picture","placeholder","playsinline","pointer-events","popover","popovertarget","position","pre","preload","progress","q","readonly","rel","required","right","role","row-gap","rows","rowspan","s","samp","sandbox","script","scroll-behavior","search","section","select","selected","sizes","small","source","span","src","srcset","step","strong","style","sub","submit","summary","sup","svg","tabindex","table","target","tbody","td","template","text-align","text-decoration","text-overflow","text-shadow","text-transform","textarea","tfoot","th","thead","time","title","top","touch-action","tr","track","transform","transition","type","u","ul","user-select","value","var","vertical-align","video","visibility","wbr","white-space","width","will-change","word-break","word-spacing","z-index",];
 // @ts-check
/// <reference lib="dom" />

/** @type { (ctx: number, chunk: number, fun_id: number, event: Event) => void } */
let event_listener = /** @type {any} */ (null);
/** @type { (fun: (ctx: number, chunk: number, fun_id: number, event: Event) => void) => void } */
export function register_event_callback (fun) {
	event_listener = fun;
}

/** @typedef { { ind: number } } Cursor */

/** @type { (buf: Uint8Array, cur: Cursor) => number } */
function decode_u8 (buf, cur) {
	return buf[cur.ind++]
}

/** decode LEB128 unsigned integer
	@type { (buf: Uint8Array, cur: Cursor) => number } */
function decode_vuint (buf, cur) {
	let res = 0, shift = 0, cond = true;
	while (cond) {
		let byte = decode_u8(buf, cur);
		res |= (byte & 0b0111_1111) << shift;
		shift += 7;
		cond = (byte & 0b1000_0000) != 0;
	}
	return res;
}

let text_decoder = new TextDecoder;
/** @type { (buf: Uint8Array, cur: Cursor) => string } */
function decode_str (buf, cur) {
	let len = decode_vuint(buf, cur);
	let text = text_decoder.decode(buf.subarray(cur.ind, cur.ind + len));
	cur.ind += len;
	return text;
}

/** @type { (buf: Uint8Array, cur: Cursor) => string } */
function decode_name (buf, cur) {
	let tag = decode_vuint(buf, cur);
	if ((tag & 1) === 1) {
		return common_names[tag >> 1]
	}
	let len = tag >> 1;
	let text = text_decoder.decode(buf.subarray(cur.ind, cur.ind + len));
	cur.ind += len;
	return text;
}

const EL_START = 0;
const EL_ID = 1;
const ATTR = 2;
const PROP = 3;
const CLASS = 4;
const STYLE = 5;
const TEXT = 6;
const NODE = 7;
const EVENT = 8;
const END = 255;

/** @type { (
	el: HTMLElement, build_codes: Uint8Array, cur: Cursor, el_refs: Element[], props: any[], nodes: Node[]
) => void } */
function construct_el (el, build_codes, cur, el_refs, props, nodes) {
	while (true) {
		let op = decode_u8(build_codes, cur);
		switch (op) {
			case EL_START: {
				let tag = decode_name(build_codes, cur);
				let child = document.createElement(tag)
				construct_el(child, build_codes, cur, el_refs, props, nodes);
				el.append(child);
				break
			}
			case EL_ID: {
				el_refs.push(el);
				break
			}
			case ATTR: {
				let attr = decode_name(build_codes, cur);
				let value = decode_str(build_codes, cur);
				el.setAttribute(attr, value);
				break
			}
			case PROP: {
				let prop = decode_str(build_codes, cur);
				let ind = decode_vuint(build_codes, cur);
				/** @type {any} */ (el)[prop] = props[ind];
				break
			}
			case CLASS: {
				let name = decode_str(build_codes, cur);
				el.classList.add(name);
				break
			}
			case STYLE: {
				let prop = decode_name(build_codes, cur);
				let value = decode_str(build_codes, cur);
				el.style.setProperty(prop, value)
				break
			}
			case TEXT: {
				let text = decode_str(build_codes, cur);
				el.append(text);
				break
			}
			case NODE: {
				let ind = decode_vuint(build_codes, cur);
				el.append(nodes[ind])
				break
			}
			case EVENT: {
				let ctx = decode_vuint(build_codes, cur);
				let chunk = decode_vuint(build_codes, cur);
				let name = decode_name(build_codes, cur);
				let fun_id = decode_vuint(build_codes, cur);
				el.addEventListener(name, (event) => event_listener(ctx, chunk, fun_id, event));
				break
			}
			case END: return;
			default: throw `binder: unkown opcode ${op}`
		}
	}
}

/** contruct an element from build codes
  @type { (
	target_el: HTMLElement, build_codes: Uint8Array, props: any[], nodes: Node[]
  ) => Element[] } */
export function construct (target_el, build_codes, props, nodes) {
	let cur = { ind: 0 };
	let el_refs = [target_el];
	construct_el(target_el, build_codes, cur, el_refs, props, nodes);
	if (cur.ind !== build_codes.length) throw "binder: excess input";
	return el_refs;
}
