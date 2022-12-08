import { invoke } from "@tauri-apps/api/tauri";
import Item from "./Item";
import { createSignal, createEffect } from "solid-js";

const timeTicker = 10;

function App() {
  const [count, setCount] = createSignal(timeTicker);
  const [items, setItems] = createSignal([]);

  createEffect(() => {
    const c = count()
    setTimeout(async () => {
      if (c === 0) {
        const response = await invoke("get_binance_ticker")
        setItems(response, { equals: false })
        setCount(timeTicker)
        return
      }
      setCount(c - 1);
    }, 1000);
  })

  return (
    <div class="grid grid-rows-2 grid-flow-row font-source h-screen bg-background p-1">
      <div class="font-light">
        <div class="grid grid-cols-4 gap-4 h-4 content-center text-xs  text-slate-400">
          <div class="col-span-2">Pair</div>
          <div class="ml-auto">Price</div>
          <div class="ml-auto">24h%</div>
        </div>
        <For each={items()}>
          {(item) => <Item symbol={item.symbol} priceChangePercent={item.change} lastPrice={item.price}/>}
        </For>
      </div>
      <div class="text-xs font-light text-slate-400">
      </div>
      <div class="text-xs font-light text-slate-400 ml-auto">
          <div class="col-span-2">Updating in: {count()}</div>
      </div>
    </div>
  );
}

export default App;
