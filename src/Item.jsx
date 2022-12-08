const buttonClass = "text-white font-bold uppercase shadow hover:shadow-md outline-none focus:outline-none ease-linear transition-all duration-150 px-2"
const greenButton = "bg-green-500 "
const redButton = "bg-red-500 "

export default function Item (props) {
    return <div class="grid grid-cols-4 gap-4 text-base h-6 content-center text-slate-700">
        <div class="col-span-2"><div>{props.symbol}</div></div>
        <div class="ml-auto"><div>{props.lastPrice}</div></div>
        <div class="ml-auto"><div class={(props.priceChangePercent.startsWith("-") ? redButton : greenButton) + buttonClass}>{props.priceChangePercent}%</div></div>
    </div>
}