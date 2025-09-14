import { TransferYearStatusFailed, TransferYearStatusSuccess } from "@/types/model"
import TotalCardMonthSuccess from "./TotalMonthSuccess"
import TotalCardYearSucccess from "./TotalYearSuccess"
import TotalCardMonthFailed from "./TotalMonthFailed"
import TotalCardYearFailed from "./TotalYearFailed"

export default function TotalCardTransferDetail({ loadingMonthStatusSuccess, currentMonthSuccess, previousMonthSuccess, monthPercentageSuccessChange, loadingYearStatusSuccess,
    currentYearSuccess,
    previousYearSuccessData,
    yearPercentageSuccessChange,
    loadingMonthStatusFailed, currentMonthFailed, previousMonthFailed, monthPercentageFailedChange,
    loadingYearStatusFailed,
    currentYearFailed,
    previousYearFailedData,
    yearPercentageFailedChange,
}: {
    loadingMonthStatusSuccess: boolean
    currentMonthSuccess: number,
    previousMonthSuccess: number,
    monthPercentageSuccessChange: number,
    loadingYearStatusSuccess: boolean,
    currentYearSuccess: number,
    previousYearSuccessData: TransferYearStatusSuccess,
    yearPercentageSuccessChange: number,
    loadingMonthStatusFailed: boolean
    currentMonthFailed: number,
    previousMonthFailed: number,
    monthPercentageFailedChange: number,
    loadingYearStatusFailed: boolean,
    currentYearFailed: number,
    previousYearFailedData: TransferYearStatusFailed,
    yearPercentageFailedChange: number
}) {
    return (
        <>
            <TotalCardMonthSuccess loadingMonthStatusSuccess={loadingMonthStatusSuccess} currentMonthSuccess={currentMonthSuccess} monthPercentageSuccessChange={monthPercentageSuccessChange} previousMonthSuccess={previousMonthSuccess} />
            <TotalCardYearSucccess loadingYearStatusSuccess={loadingYearStatusSuccess} currentYearSuccess={currentYearSuccess}
                previousYearSuccessData={previousYearSuccessData}
                yearPercentageSuccessChange={yearPercentageSuccessChange}
            />
            <TotalCardMonthFailed loadingMonthStatusFailed={loadingMonthStatusFailed} currentMonthFailed={currentMonthFailed} monthPercentageFailedChange={monthPercentageFailedChange} previousMonthFailed={previousMonthFailed} />
            <TotalCardYearFailed loadingYearStatusFailed={loadingYearStatusFailed} currentYearFailed={currentYearFailed}
                previousYearFailedData={previousYearFailedData}
                yearPercentageFailedChange={yearPercentageFailedChange}
            />
        </>
    )
}