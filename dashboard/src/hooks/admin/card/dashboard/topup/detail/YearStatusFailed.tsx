import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useTopupStore from "@/store/topup/topup";
import { useCallback } from "react";

export default function useYearStatusFailedTopupDetail(currentYear: number, toast: any) {
    const {
        findYearStatusFailedByCardNumber,
        yearStatusFailed,
        loadingYearStatusFailed,
        setLoadingYearStatusFailed,
        setErrorYearStatusFailed,
    } = useTopupStore();

    const currentYearFailedData = yearStatusFailed?.find(
        (balance) => balance.year === currentYear.toString(),
    );

    const previousYearFailedData = yearStatusFailed?.find(
        (balance) => balance.year === (currentYear - 1).toString(),
    );

    const currentYearFailed = currentYearFailedData?.total_amount || 0;
    const previousYearFailed = previousYearFailedData?.total_amount || 0;

    const yearPercentageFailedChange = calculatePercentageChange(
        currentYearFailed,
        previousYearFailed,
    );

    const fetchYearlyFailed = useCallback(
        async (year: number, card_number: string) => {
            try {
                setLoadingYearStatusFailed(true);
                setErrorYearStatusFailed(null);

                await findYearStatusFailedByCardNumber(toast, year, card_number);
            } catch (error) {
                setErrorYearStatusFailed("Failed to fetch yearly balance");
            } finally {
                setLoadingYearStatusFailed(false);
            }
        },
        [
            findYearStatusFailedByCardNumber,
            setLoadingYearStatusFailed,
            setLoadingYearStatusFailed,
        ],
    );

    const debouncedFetchYearlyFailed = debounce(fetchYearlyFailed, 300);

    return {
        debouncedFetchYearlyFailed,
        loadingYearStatusFailed,
        currentYearFailedData,
        previousYearFailedData,
        currentYearFailed,
        previousYearFailed,
        yearPercentageFailedChange,
        findYearStatusFailedByCardNumber
    }
}