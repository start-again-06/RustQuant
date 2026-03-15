import Link from "next/link"

export default function Navbar(){

  return(

    <nav>

      <Link href="/dashboard">Dashboard</Link>
      <Link href="/predictions">Predictions</Link>
      <Link href="/portfolio">Portfolio</Link>
      <Link href="/backtest">Backtest</Link>

    </nav>
  )
}
