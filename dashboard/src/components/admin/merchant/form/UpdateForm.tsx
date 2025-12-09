import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { debounce } from "@/helpers/debounce";
import { useToast } from "@/hooks/use-toast";
import {
  UpdateMerchantFormValues,
  updateMerchantRequestSchema,
} from "@/schemas";
import AsyncSelect from "react-select/async";
import useUserStore from "@/store/user/user";
import {
  Select as ShadcnSelect,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { MerchantUpdateFormProps } from "@/types/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { FindAllUser, FindByIdUser } from "@/types/domain/request";

interface UserOption {
  value: string;
  label: string;
}

const merchantStatusOptions = [
  { value: "inactive", label: "Inactive" },
  { value: "active", label: "Active" },
  { value: "suspend", label: "Suspend" },
];

const UpdateMerchantForm = forwardRef<HTMLFormElement, MerchantUpdateFormProps>(
  ({ onSubmit, defaultValues }, ref) => {
    const {
      register,
      control,
      handleSubmit,
      formState: { errors },
    } = useForm<UpdateMerchantFormValues>({
      resolver: zodResolver(updateMerchantRequestSchema),
      defaultValues: defaultValues || {
        user_id: {
          value: "",
          label: "",
        },
        name: "",
        status: "",
      },
    });
    const [userOptions, setUserOptions] = useState<UserOption[]>([]);

    const { toast } = useToast();

    const {
      users,
      loadingGetUsers,
      setLoadingGetUsers,
      findAllUsers,
      findById,
      user,
    } = useUserStore();

    useEffect(() => {
      const fetchInitialUsers = async () => {
        setLoadingGetUsers(true);
        try {
          const searchReq: FindAllUser = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllUsers(searchReq);

          if (users) {
            const options = users.map((user) => ({
              value: user.id.toString(),
              label: `${user.firstname} ${user.lastname}` || user.email,
            }));

            if (defaultValues?.user_id) {
              const isUserInList = options.some(
                (opt) => opt.value === defaultValues.user_id.toString(),
              );
              if (!isUserInList) {
                const req: FindByIdUser = {
                  toast,
                  id: Number(defaultValues.user_id.value),
                };

                await findById(req);
                if (user) {
                  options.unshift({
                    value: user.id.toString(),
                    label: `${user.firstname} ${user.lastname}` || user.email,
                  });
                }
              }
            }

            setUserOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial users:", error);
        } finally {
          setLoadingGetUsers(false);
        }
      };

      fetchInitialUsers();
    }, []);

    const loadOptions = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: UserOption[]) => void,
        ) => {
          setLoadingGetUsers(true);
          try {
            const searchReq: FindAllUser = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllUsers(searchReq);

            if (users) {
              const filteredOptions = users
                .filter(
                  (user) =>
                    `${user.firstname} ${user.lastname}`
                      .toLowerCase()
                      .includes(inputValue.toLowerCase()) ||
                    user.email.toLowerCase().includes(inputValue.toLowerCase()),
                )
                .map((user) => ({
                  value: user.id.toString(),
                  label: `${user.firstname} ${user.lastname}` || user.email,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch users:", error);
          } finally {
            setLoadingGetUsers(false);
          }
        },
        500,
      ),
      [users, findAllUsers, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="userId" className="text-sm font-medium">
            User ID
          </Label>
          <Controller
            name="user_id"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptions}
                defaultOptions={userOptions}
                isLoading={loadingGetUsers}
                placeholder="Select a user"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.user_id ? "#ef4444" : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={
                  userOptions.find(
                    (option) =>
                      option.value ===
                      (typeof field.value === "object"
                        ? field.value?.value
                        : field.value),
                  ) || {
                    value: defaultValues?.user_id?.value || "",
                    label: defaultValues?.user_id?.label || "Select a user",
                  }
                }
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption
                      ? {
                          value: selectedOption.value,
                          label: selectedOption.label,
                        }
                      : null,
                  );
                }}
              />
            )}
          />
          {errors.user_id && (
            <p className="text-red-500 text-sm mt-1">
              {errors.user_id.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="name" className="text-sm font-medium">
            Name
          </Label>
          <Input
            id="name"
            type="text"
            placeholder="Enter merchant name"
            className="mt-1"
            {...register("name")}
          />
          {errors.name && (
            <p className="text-red-500 text-sm mt-1">{errors.name.message}</p>
          )}
        </div>

        <div>
          <Label htmlFor="cardType" className="text-sm font-medium">
            Merchant Status
          </Label>
          <Controller
            name="status"
            control={control}
            render={({ field }) => (
              <ShadcnSelect onValueChange={field.onChange} value={field.value}>
                <SelectTrigger className="w-full mt-1">
                  <SelectValue placeholder="Select card type" />
                </SelectTrigger>
                <SelectContent>
                  {merchantStatusOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </ShadcnSelect>
            )}
          />
          {errors.status && (
            <p className="text-red-500 text-sm mt-1">{errors.status.message}</p>
          )}
        </div>
      </form>
    );
  },
);

UpdateMerchantForm.displayName = "UpdateMerchantForm";

export default UpdateMerchantForm;
