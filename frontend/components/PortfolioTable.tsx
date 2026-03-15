export default function PortfolioTable(){

  const data = [
    {symbol:"AAPL",qty:10},
    {symbol:"TSLA",qty:5}
  ]

  return(

    <table>

      <thead>
        <tr>
          <th>Symbol</th>
          <th>Quantity</th>
        </tr>
      </thead>

      <tbody>

        {data.map((row,i)=>(
          <tr key={i}>
            <td>{row.symbol}</td>
            <td>{row.qty}</td>
          </tr>
        ))}

      </tbody>

    </table>
  )
}
