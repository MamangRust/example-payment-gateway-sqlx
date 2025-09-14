import { MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount } from "@/types/model"
import TotalCardMonthAmountApiKey from "./TotalMonthAmount"
import TotalCardYearAmountApiKey from "./TotalYearAmount"

export default function TotalCardMerchantApiKey({
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
            <TotalCardMonthAmountApiKey loadingMonthTotalAmount={loadingMonthTotalAmount} currentMonthData={currentMonthData} currentMonthBalance={currentMonthBalance} previousMonthData={previousMonthData} monthPercentageChange={monthPercentageChange} />
            <TotalCardYearAmountApiKey loadingYearTotalAmount={loadingYearTotalAmount} currentYearData={currentYearData} currentYearBalance={currentYearBalance} previousYearData={previousYearData} yearPercentageChange={yearPercentageChange} />
        </>
    )
}