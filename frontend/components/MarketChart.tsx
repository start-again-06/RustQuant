import { Line } from "react-chartjs-2"

export default function MarketChart(){

  const data = {

    labels:["1","2","3","4"],
    datasets:[
      {
        label:"Price",
        data:[100,102,101,105]
      }
    ]
  }

  return <Line data={data}/>
}
