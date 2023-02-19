import * as React from "react"
 
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { Transactions } from "@/types/transactions"

export default function TransactionsScroll({ data }: Transactions) {
    const weekday = ["Sunday","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"]
    return (
        <>
            <h4 className="pt-5 mb-4 text-lg text-center font-bold leading-none">Transactions</h4>
            <ScrollArea className="h-96 rounded-md border border-slate-100 dark:border-slate-700">
                <div className="p-4">
                    <div className="pb-5">
                        <div className="grid grid-cols-3 text-center">
                            <div>Date</div>
                            <div>Merchant</div>
                            <div>Amount</div>
                        </div>
                    <div className="w-full border-b-4 border-b-slate-200 bg-white dark:border-b-slate-700 dark:bg-slate-900"></div>
                    <div className="pt-4"></div>
                        {data.items.map((purchase) => (
                        <React.Fragment>
                            <div className="text-sm" key={purchase.id}>
                                <div>
                                    <div className="grid grid-cols-3 text-center">
                                        <div>{weekday[new Date(purchase.date).getDay()]} {new Date(purchase.date).getMonth() + 1}/{new Date(purchase.date).getDate()}</div>
                                        <div className="">{purchase.merchant_name}</div>
                                        <div className="">{purchase.amount}</div>
                                    </div>
                                </div>
                            </div>
                            <Separator className="my-2" />
                        </React.Fragment>
                        ))}
                    </div>
                </div>
            </ScrollArea>
        </>
    )
}