import api from "./api"

export async function getMarketData(){

  const res = await api.get("/market")

  return res.data
}
