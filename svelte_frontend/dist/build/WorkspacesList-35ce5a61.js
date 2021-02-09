
(function(l, r) { if (l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (window.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(window.document);
import { S as SvelteComponentDev, i as init, s as safe_not_equal, d as dispatch_dev, v as validate_slots, e as element, o as create_component, g as space, h as attr_dev, a as add_location, b as insert_dev, p as mount_component, k as append_dev, q as noop, m as transition_in, n as transition_out, f as detach_dev, r as destroy_component } from './main-133695a0.js';
import Item from './Item-ec4e2314.js';
import { P as Plus } from './Plus-9a544b69.js';

/* src/pages/WorkspacesList.svelte generated by Svelte v3.32.1 */
const file = "src/pages/WorkspacesList.svelte";

function create_fragment(ctx) {
	let list;
	let item0;
	let t0;
	let item1;
	let t1;
	let newWorkSpace;
	let button;
	let plus;
	let t2;
	let h2;
	let current;

	item0 = new Item({
			props: {
				courseName: "Kurs 1",
				courseText: "Dålig kurs"
			},
			$$inline: true
		});

	item0.$on("onClick", handleClick);

	item1 = new Item({
			props: {
				courseName: "Kurs 2",
				courseText: "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc."
			},
			$$inline: true
		});

	item1.$on("onClick", handleClick);
	plus = new Plus({ $$inline: true });

	const block = {
		c: function create() {
			list = element("list");
			create_component(item0.$$.fragment);
			t0 = space();
			create_component(item1.$$.fragment);
			t1 = space();
			newWorkSpace = element("newWorkSpace");
			button = element("button");
			create_component(plus.$$.fragment);
			t2 = space();
			h2 = element("h2");
			h2.textContent = "New Workspace";
			attr_dev(h2, "class", "svelte-1fk4wj8");
			add_location(h2, file, 17, 6, 1141);
			attr_dev(button, "class", "svelte-1fk4wj8");
			add_location(button, file, 15, 4, 1111);
			add_location(newWorkSpace, file, 14, 2, 1092);
			attr_dev(list, "class", "svelte-1fk4wj8");
			add_location(list, file, 7, 0, 169);
		},
		l: function claim(nodes) {
			throw new Error("options.hydrate only works if the component was compiled with the `hydratable: true` option");
		},
		m: function mount(target, anchor) {
			insert_dev(target, list, anchor);
			mount_component(item0, list, null);
			append_dev(list, t0);
			mount_component(item1, list, null);
			append_dev(list, t1);
			append_dev(list, newWorkSpace);
			append_dev(newWorkSpace, button);
			mount_component(plus, button, null);
			append_dev(button, t2);
			append_dev(button, h2);
			current = true;
		},
		p: noop,
		i: function intro(local) {
			if (current) return;
			transition_in(item0.$$.fragment, local);
			transition_in(item1.$$.fragment, local);
			transition_in(plus.$$.fragment, local);
			current = true;
		},
		o: function outro(local) {
			transition_out(item0.$$.fragment, local);
			transition_out(item1.$$.fragment, local);
			transition_out(plus.$$.fragment, local);
			current = false;
		},
		d: function destroy(detaching) {
			if (detaching) detach_dev(list);
			destroy_component(item0);
			destroy_component(item1);
			destroy_component(plus);
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

function handleClick(event) {
	alert(event.detail.text);
}

function instance($$self, $$props, $$invalidate) {
	let { $$slots: slots = {}, $$scope } = $$props;
	validate_slots("WorkspacesList", slots, []);
	const writable_props = [];

	Object.keys($$props).forEach(key => {
		if (!~writable_props.indexOf(key) && key.slice(0, 2) !== "$$") console.warn(`<WorkspacesList> was created with unknown prop '${key}'`);
	});

	$$self.$capture_state = () => ({ Item, Plus, handleClick });
	return [];
}

class WorkspacesList extends SvelteComponentDev {
	constructor(options) {
		super(options);
		init(this, options, instance, create_fragment, safe_not_equal, {});

		dispatch_dev("SvelteRegisterComponent", {
			component: this,
			tagName: "WorkspacesList",
			options,
			id: create_fragment.name
		});
	}
}

export default WorkspacesList;
//# sourceMappingURL=WorkspacesList-35ce5a61.js.map
