import { SaldoMonthTotalBalance, SaldoYearTotalBalance } from "@/types/model"
import TotalCardMonthBalance from "./TotalMonthBalance"
import TotalCardYearBalance from "./TotalYearBalance"

export default function TotalCardSaldo({
    loadingMonthTotalBalance, currentMonthData, currentMonthBalance, previousMonthData, monthPercentageChange, loadingYearTotalBalance,
    currentYearData,
    currentYearBalance,
    previousYearData,
    yearPercentageChange }: {
        loadingMonthTotalBalance: boolean
        currentMonthData: SaldoMonthTotalBalance,
        currentMonthBalance: number,
        previousMonthData: SaldoMonthTotalBalance,
        monthPercentageChange: number,
        loadingYearTotalBalance: boolean,
        currentYearData: SaldoYearTotalBalance,
        currentYearBalance: number,
        previousYearData: SaldoYearTotalBalance,
        yearPercentageChange: number
    }) {
    return (
        <>
            <TotalCardMonthBalance loadingMonthTotalBalance={loadingMonthTotalBalance} currentMonthData={currentMonthData} currentMonthBalance={currentMonthBalance} previousMonthData={previousMonthData} monthPercentageChange={monthPercentageChange} />
            <TotalCardYearBalance loadingYearTotalBalance={loadingYearTotalBalance} currentYearData={currentYearData} currentYearBalance={currentYearBalance} previousYearData={previousYearData} yearPercentageChange={yearPercentageChange} />
        </>
    )
}