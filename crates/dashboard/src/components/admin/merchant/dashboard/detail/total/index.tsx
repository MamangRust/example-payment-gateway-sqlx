import { MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount } from "@/types/model"
import TotalCardMonthAmountDetail from "./TotalMonthAmount"
import TotalCardYearAmountDetail from "./TotalYearAmount"

export default function TotalCardMerchantDetail({
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
            <TotalCardMonthAmountDetail loadingMonthTotalAmount={loadingMonthTotalAmount} currentMonthData={currentMonthData} currentMonthBalance={currentMonthBalance} previousMonthData={previousMonthData} monthPercentageChange={monthPercentageChange} />
            <TotalCardYearAmountDetail loadingYearTotalAmount={loadingYearTotalAmount} currentYearData={currentYearData} currentYearBalance={currentYearBalance} previousYearData={previousYearData} yearPercentageChange={yearPercentageChange} />
        </>
    )
}