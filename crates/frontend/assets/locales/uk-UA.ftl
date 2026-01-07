#Auth
log-in = Увійти
log-out = Вийти

error-authentication = Невірні облікові дані!
error-authorization = Невірний рівень доступу!


#Breadcrumbs
breadcrumb-home = ГОЛОВНЕ МЕНЮ
breadcrumb-cadets = РЕЄСТР КУРСАНТІВ
breadcrumb-cadet-courses = РЕЄСТР КУРСІВ
breadcrumb-users = КОРИСТУВАЧІ СИСТЕМИ
breadcrumb-login = МЕНЮ ВХОДУ


#Actions
actions = Дії
apply = Застосувати
cancel = Відмінити
back = Назад
clear = Очистити
continue = Продовжити
create = Створити
delete = Видалити
export = Експортувати
import = Імпортувати
ok = Зрозуміло
open = Відкрити
save = Зберегти
update = Оновити
show = Показати


#Dialog
dialog-delete-confirmation = Бажаєте видалити?
dialog-tax-number-passport-format = Введений РНОКПП не пройшов перевірку. Якщо особа використовує номер паспорта як податковий ідентифікатор, ви можете продовжити. Бажаєте зберегти?
dialog-tax-number-card-id-format = Введений РНОКПП не пройшов перевірку. Якщо особа використовує номер ID картки як податковий ідентифікатор, ви можете продовжити. Бажаєте зберегти?
dialog-tax-number-unknown-format = Введений РНОКПП не пройшов перевірку. Бажаєте зберегти?


# System
system-user = Поточний користувач
system-user-role-name = Рівень доступу
system-name = Система «Вишкіл»
system-department-name = ОК Північ управління підготовки
system-copyright = © 2026 ЗСУ • Система «Вишкіл» • ОК Північ управління підготовки
system-status = Система не у мережі (Локальний вузол)
system-version = Версія ПЗ: 1.0.0
system-restriction = Для службового використання


#Cadet Filter
search-filters = Фільтри пошуку
birth-date-after = Дата народження (з)
birth-date-before = Дата народження (по)
start-course-date-after = Початок навчання (з)
start-course-date-before = Початок навчання (по)
end-course-date-after = Закінчення навчання (з)
end-course-date-before = Закінчення навчання (по)


# Error
error-resource-conflict = { $resource-name } з { $unique-key-name } { $unique-key-value } вже існує!
error-resource-not-found = { $resource-name } з ідентифікатором { $resource-id } не існує!
error-internal = Сталася неочікувана помилка: зверніться до адміністратора системи!


# INFO
info-show-encryption-key = Ключ шифрування вашої інсталяції: { $encryption-key }


# Validation
error-validation = Помилка валідації
error-validation-blank-string = не може бути пустим
error-validation-invalid-m-d-y-date = дата не відповідає формату ММ/ДД/РРРР
error-validation-inconsistent-tax-number-birth-date-or-passport-number = РНОКПП не відповідає даті народження і не є номером паспорта
error-validation-invalid-tax-number = не відповідає формату РНОКПП
error-validation-invalid-full-name = ПІБ не відповідає формату: прізвище, ім'я, по батькові


# CADET
last-name = Прізвище
first-name = Імʼя
middle-name = По батькові
tax-number = РНОКПП
birth-date = Дата народження

cadet = Курсант
cadets = Реєстр курсантів
cadet-management = керування базою випускників курсів

enter-first-name = Введіть імʼя
enter-middle-name = Введіть по батькові
enter-last-name = Введіть прізвище
enter-tax-number = Введіть РНОКПП (ІПН)
enter-date = Введіть дату у форматі ММ/ДД/РРРР


# CADET COURSE
military-rank = Військове звання
full-name = ПІБ
source-unit = З якої військової частини або ТЦК та СП прибув
specialty-name = Посада
specialty-code = ВОС
specialty-mos-code = Код посади
category = Найменування рівня військової підготовки
training-location = Місце навчання
start-date = Дата початку
end-date = Дата закінчення
completion-order-number = Дата та № наказу
completion-certificate-number = № диплому
notes = Примітка

enter-military-rank = Введіть військове звання
enter-source-unit = Введіть частину або ТЦК та СП
enter-specialty-name = Введіть посаду
enter-specialty-code = Введіть ВОС
enter-specialty-mos-code = Введіть код посади
enter-category = Введіть найменування рівня освіти
enter-training-location = Введіть місце навчання
enter-completion-order-number = Введіть дату у форматі ДД.ММ.РРРР та № наказу
enter-completion-certificate-number = Введіть № диплому
enter-notes = Введіть примітку

cadet-course = Курс
cadet-courses = Реєстр курсів
cadet-course-management = керування базою пройдених курсів

number-of-completed-cadet-courses = Кількість підготовлених курсантів

export-import-failed-cadet-course-entry-file-name = помилкові_курси
export-cadet-course-entry-file-name = курси
info-import-cadet-course-entry-succeeded = Імпортування курсів завершено: успішних { $number-of-succeeded }
dialog-import-export-failed-cadet-course-entry = Імпортування курсів завершено: успішних { $number-of-succeeded }, помилкових { $number-of-failed }. Бажаєте експортувати помилкові?

error-cadet-course-search = Сталася помилка під час отримання курсів!
error-cadet-course-file-read = Сталася помилка під час читання файлу курсів: перевірте заголовки файлу!
error-cadet-course-import = Сталася помилка під час імпорту курсів!
error-cadet-course-export = Сталася помилка під час експорту курсів!


# USER
login = Логін
password = Пароль
role = Роль
role-name = { $name ->
[admin] Адміністратор
[writer] Редактор
[reader] Читач
*[other] Невідомий
}

users = Користувачі системи
user-management = Керування обліковими записами

enter-login = Введіть логін
enter-password = Введіть пароль


# SELECT
select-option = "Оберіть варіант"