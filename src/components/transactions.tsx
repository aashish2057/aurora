import * as React from "react"
import { pb } from "@/pages/api/pocketbase/pocketbase"

import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"

export async function getServerSideProps() {
    const response = await fetch("/api/pocketbase/getTransactions", {
        method: "GET",
      })
    const data = await response.json()
    console.log(response)
    return {props: {"hello": "1"}}
}

export default function Transactions({resultList}) {
    
  console.log(resultList)
  
  return (
    <>
        {resultList}
      <ScrollArea className="h-72 w-48 rounded-md border border-slate-100 dark:border-slate-700">
        <div className="p-4">
          <h4 className="mb-4 text-sm font-medium leading-none">Tags</h4>
          {/* {resultList?.items.map((tag) => (
            <React.Fragment>
              <div className="text-sm" key={tag}>
                {tag}
              </div>
              <Separator className="my-2" />
            </React.Fragment>
          ))} */}
        </div>
      </ScrollArea>
    </>
  )
}




