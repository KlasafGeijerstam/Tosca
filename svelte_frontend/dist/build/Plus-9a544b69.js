
(function(l, r) { if (l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (window.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(window.document);
import { S as SvelteComponentDev, i as init, s as safe_not_equal, d as dispatch_dev, v as validate_slots, w as svg_element, h as attr_dev, a as add_location, b as insert_dev, k as append_dev, q as noop, f as detach_dev } from './main-133695a0.js';

/* node_modules/tabler-icons-svelte/dist/Plus.svelte generated by Svelte v3.32.1 */

const file = "node_modules/tabler-icons-svelte/dist/Plus.svelte";

function create_fragment(ctx) {
	let svg;
	let path;
	let line0;
	let line1;

	const block = {
		c: function create() {
			svg = svg_element("svg");
			path = svg_element("path");
			line0 = svg_element("line");
			line1 = svg_element("line");
			attr_dev(path, "stroke", "none");
			attr_dev(path, "d", "M0 0h24v24H0z");
			attr_dev(path, "fill", "none");
			add_location(path, file, 19, 2, 398);
			attr_dev(line0, "x1", "12");
			attr_dev(line0, "y1", "5");
			attr_dev(line0, "x2", "12");
			attr_dev(line0, "y2", "19");
			add_location(line0, file, 20, 2, 452);
			attr_dev(line1, "x1", "5");
			attr_dev(line1, "y1", "12");
			attr_dev(line1, "x2", "19");
			attr_dev(line1, "y2", "12");
			add_location(line1, file, 21, 2, 494);
			attr_dev(svg, "xmlns", "http://www.w3.org/2000/svg");
			attr_dev(svg, "class", "icon icon-tabler icon-tabler-plus");
			attr_dev(svg, "width", /*size*/ ctx[0]);
			attr_dev(svg, "height", /*size*/ ctx[0]);
			attr_dev(svg, "viewBox", "0 0 24 24");
			attr_dev(svg, "stroke", /*color*/ ctx[1]);
			attr_dev(svg, "stroke-width", /*strokeWidth*/ ctx[2]);
			attr_dev(svg, "fill", "none");
			attr_dev(svg, "stroke-linecap", "round");
			attr_dev(svg, "stroke-linejoin", "round");
			add_location(svg, file, 6, 0, 117);
		},
		l: function claim(nodes) {
			throw new Error("options.hydrate only works if the component was compiled with the `hydratable: true` option");
		},
		m: function mount(target, anchor) {
			insert_dev(target, svg, anchor);
			append_dev(svg, path);
			append_dev(svg, line0);
			append_dev(svg, line1);
		},
		p: function update(ctx, [dirty]) {
			if (dirty & /*size*/ 1) {
				attr_dev(svg, "width", /*size*/ ctx[0]);
			}

			if (dirty & /*size*/ 1) {
				attr_dev(svg, "height", /*size*/ ctx[0]);
			}

			if (dirty & /*color*/ 2) {
				attr_dev(svg, "stroke", /*color*/ ctx[1]);
			}

			if (dirty & /*strokeWidth*/ 4) {
				attr_dev(svg, "stroke-width", /*strokeWidth*/ ctx[2]);
			}
		},
		i: noop,
		o: noop,
		d: function destroy(detaching) {
			if (detaching) detach_dev(svg);
		}
	};

	dispatch_dev("SvelteRegisterBlock", {
		block,
		id: create_fragment.name,
		type: "component",
		source: "",
		ctx
	});

	return block;
}

function instance($$self, $$props, $$invalidate) {
	let { $$slots: slots = {}, $$scope } = $$props;
	validate_slots("Plus", slots, []);
	let { size = 24 } = $$props;
	let { color = "currentColor" } = $$props;
	let { strokeWidth = 2 } = $$props;
	const writable_props = ["size", "color", "strokeWidth"];

	Object.keys($$props).forEach(key => {
		if (!~writable_props.indexOf(key) && key.slice(0, 2) !== "$$") console.warn(`<Plus> was created with unknown prop '${key}'`);
	});

	$$self.$$set = $$props => {
		if ("size" in $$props) $$invalidate(0, size = $$props.size);
		if ("color" in $$props) $$invalidate(1, color = $$props.color);
		if ("strokeWidth" in $$props) $$invalidate(2, strokeWidth = $$props.strokeWidth);
	};

	$$self.$capture_state = () => ({ size, color, strokeWidth });

	$$self.$inject_state = $$props => {
		if ("size" in $$props) $$invalidate(0, size = $$props.size);
		if ("color" in $$props) $$invalidate(1, color = $$props.color);
		if ("strokeWidth" in $$props) $$invalidate(2, strokeWidth = $$props.strokeWidth);
	};

	if ($$props && "$$inject" in $$props) {
		$$self.$inject_state($$props.$$inject);
	}

	return [size, color, strokeWidth];
}

class Plus extends SvelteComponentDev {
	constructor(options) {
		super(options);
		init(this, options, instance, create_fragment, safe_not_equal, { size: 0, color: 1, strokeWidth: 2 });

		dispatch_dev("SvelteRegisterComponent", {
			component: this,
			tagName: "Plus",
			options,
			id: create_fragment.name
		});
	}

	get size() {
		throw new Error("<Plus>: Props cannot be read directly from the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}

	set size(value) {
		throw new Error("<Plus>: Props cannot be set directly on the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}

	get color() {
		throw new Error("<Plus>: Props cannot be read directly from the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}

	set color(value) {
		throw new Error("<Plus>: Props cannot be set directly on the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}

	get strokeWidth() {
		throw new Error("<Plus>: Props cannot be read directly from the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}

	set strokeWidth(value) {
		throw new Error("<Plus>: Props cannot be set directly on the component instance unless compiling with 'accessors: true' or '<svelte:options accessors/>'");
	}
}

export { Plus as P };
//# sourceMappingURL=Plus-9a544b69.js.map
