import { Transactions } from "@/types/transactions"

import React from "react"
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Tooltip,
    elements
  } from 'chart.js';
  import { Line } from 'react-chartjs-2';
  import { faker } from '@faker-js/faker';
  
  ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Tooltip
  );
  
  export const options = {
    response: true,
    maintainAspectRation: false,
    plugins: {
      
    },
    scales: {
        x: {
            display: false
        },
        y: {
            display: false
        },
    },
    elements: {
        point: {
            radius: 0
        }
    }
  };
  
export function SpendingChart({data}: Transactions) {
    // see daily spend by date
    // let spendingData: any={};
    // spendingData = data.items.reduce((iw, curr) => {
    // iw[new Date(curr.date).toDateString()] ? iw[new Date(curr.date).toDateString()] += curr.amount : iw[new Date(curr.date).toDateString()] = curr.amount;
    //     return iw;
    // }, spendingData);

    
    let spendingData: any=[];
    spendingData = data.items.reduce((iw, curr) => {
    iw[new Date(curr.date).toDateString()] ? iw[new Date(curr.date).toDateString()] += curr.amount : spendingData.push(curr.amount);
        return iw;
    }, spendingData);

    let sum = 0
    const dailySpending = spendingData.map(value => sum += value)


    let labels: any=[];
    labels = data.items.reduce((iw, curr) => {
     labels.push(new Date(curr.date).getMonth() + 1 + "/" + new Date(curr.date).getDate() + "/" + new Date(curr.date).getFullYear());
        return iw;
    }, labels);


    // const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
    const Data = [20, 30, 40, 60, 30, 50, 20]
  
    const spendData = {
        labels,
        datasets: [
        {
            label: 'Dataset 1',
            data: dailySpending,
            borderColor: 'rgb(255, 99, 132)',
            backgroundColor: 'rgba(255, 99, 132, 0.5)',
        }
        ],
    };
    
    return (

        <>
            <h4 className="pt-5 mb-4 text-lg text-center font-bold leading-none">Spending</h4>
            <div className="h-96 rounded-md border border-slate-100 dark:border-slate-700 flex justify-center">
                <Line options={options} data={spendData} />
            </div>
        </>
    )
}
    