import api from "./api"

export async function placeTrade(symbol:string,qty:number){

  const res = await api.post("/trade",{
    symbol,
    quantity:qty
  })

  return res.data
}
