import * as React from "react"
 
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { Transactions } from "@/types/transactions"

export default function TransactionsScroll({ data }: Transactions) {
    return (
        <>
            <ScrollArea className="h-96 rounded-md border border-slate-100 dark:border-slate-700">
                <div className="p-4">
                    <h4 className="mb-4 text-lg text-center font-bold leading-none">Transactions</h4>
                    <div className="w-full border-b border-b-slate-200 bg-white dark:border-b-slate-700 dark:bg-slate-900"></div>
                    <div className="pt-5">
                        {data.items.map((purchase) => (
                        <React.Fragment>
                            <div className="text-sm" key={purchase.id}>
                                <div>
                                    <div className="flex space-x-5">
                                        <div>Amount</div>
                                        <div>Merchant</div>
                                    </div>
                                    <div className="flex space-x-5">
                                        <div>{purchase.amount}</div>
                                        <div>{purchase.merchant_name}</div>
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