import { MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount } from "@/types/model"
import TotalCardMonthAmount from "./TotalMonthAmount"
import TotalCardYearAmount from "./TotalYearAmount"

export default function TotalCardMerchant({
    loadingMonthTotalAmount, currentMonthData, currentMonthBalance, previousMonthData, monthPercentageChange, loadingYearTotalAmount,
    currentYearData,
    currentYearBalance,
    previousYearData,
    yearPercentageChange }: {
        loadingMonthTotalAmount: boolean
        currentMonthData: MerchantMonthlyTotalAmount,
        currentMonthBalance: number,
        previousMonthData: MerchantMonthlyTotalAmount,
        monthPercentageChange: number,
        loadingYearTotalAmount: boolean,
        currentYearData: MerchantYearlyTotalAmount,
        currentYearBalance: number,
        previousYearData: MerchantYearlyTotalAmount,
        yearPercentageChange: number
    }) {
    return (
        <>
            <TotalCardMonthAmount loadingMonthTotalAmount={loadingMonthTotalAmount} currentMonthData={currentMonthData} currentMonthBalance={currentMonthBalance} previousMonthData={previousMonthData} monthPercentageChange={monthPercentageChange} />
            <TotalCardYearAmount loadingYearTotalAmount={loadingYearTotalAmount} currentYearData={currentYearData} currentYearBalance={currentYearBalance} previousYearData={previousYearData} yearPercentageChange={yearPercentageChange} />
        </>
    )
}