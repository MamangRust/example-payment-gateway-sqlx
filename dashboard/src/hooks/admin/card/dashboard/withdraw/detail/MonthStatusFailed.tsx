import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import { monthNames } from "@/helpers/month";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback } from "react";


export default function useMonthStatusFailedWithdrawDetail(currentMonths: number, currentYear: number, previousMonth: number, previousYear: number, toast: any) {
    const {
        findMonthStatusFailedByCardNumber,
        monthStatusFailed,
        loadingMonthStatusFailed,
        setLoadingMonthStatusFailed,
        setErrorMonthStatusFailed,
    } = useWithdrawStore();

    const currentMonthFailedData = monthStatusFailed?.find(
        (balance) =>
            balance.month === monthNames[currentMonths] &&
            balance.year === currentYear.toString(),
    );

    const previousMonthFailedData = monthStatusFailed?.find(
        (balance) =>
            balance.month === monthNames[previousMonth] &&
            balance.year === previousYear.toString(),
    );

    const currentMonthFailed = currentMonthFailedData?.total_amount || 0;
    const previousMonthFailed = previousMonthFailedData?.total_amount || 0;

    const monthPercentageFailedChange = calculatePercentageChange(
        currentMonthFailed,
        previousMonthFailed,
    );


    const fetchMonthlyFailed = useCallback(
        async (year: number, month: number,card_number: string) => {
            try {
                setLoadingMonthStatusFailed(true);
                setErrorMonthStatusFailed(null);

                await findMonthStatusFailedByCardNumber(toast, year, month,card_number);
            } catch (error) {
                setErrorMonthStatusFailed("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthStatusFailed(false);
            }
        },
        [
            findMonthStatusFailedByCardNumber,
            setLoadingMonthStatusFailed,
            setErrorMonthStatusFailed,
        ],
    );

    const debouncedFetchMonthlyFailed = debounce(fetchMonthlyFailed, 300);

    return{
        monthPercentageFailedChange,
        loadingMonthStatusFailed,
        currentMonthFailed,
        previousMonthFailed,
        debouncedFetchMonthlyFailed
    }
}