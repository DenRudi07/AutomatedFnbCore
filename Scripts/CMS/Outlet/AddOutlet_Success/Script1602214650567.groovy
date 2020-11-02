import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('CMS/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(3)

WebUI.refresh()

WebUI.delay(1)

WebUI.click(findTestObject('CMS/Outlet/MenuOutlet'))

WebUI.delay(1)

WebUI.click(findTestObject('CMS/Outlet/AddOutlet'))

WebUI.click(findTestObject('CMS/Outlet/Label_BrandName'))

WebUI.sendKeys(findTestObject('CMS/Outlet/Label_BrandName'), 'a')

WebUI.sendKeys(findTestObject('CMS/Outlet/Label_BrandName'), Keys.chord(Keys.ARROW_DOWN, Keys.ENTER))

WebUI.setText(findTestObject('CMS/Outlet/OutletID'), 'OutletAuto1')

WebUI.delay(2)

String Ms = WebUI.verifyTextPresent('Sorry Outlet ID is already exists, please use another one', false)

if (Ms.equals('true')) {
	int RN

	RN = ((Math.random() * 500) as int)

	WebUI.setText(findTestObject('CMS/Outlet/OutletID'), '' + RN)
}

WebUI.setText(findTestObject('CMS/Outlet/OutletName'), 'Outlet Kopi')

WebUI.click(findTestObject('CMS/Outlet/Btn_ContinueGeneralInfo'))

WebUI.click(findTestObject('CMS/Outlet/Label_Location'))

WebUI.click(findTestObject('CMS/Outlet/Location_Denpasar'))

WebUI.setText(findTestObject('CMS/Outlet/Label_Longitude'), '180')

WebUI.setText(findTestObject('CMS/Outlet/Label_Latitude'), '90')

WebUI.setText(findTestObject('CMS/Outlet/Label_Address'), 'Denpasar Bali')

WebUI.click(findTestObject('CMS/Outlet/Btn_ContinueLocation'))

WebUI.setText(findTestObject('CMS/Outlet/PhoneNumber'), '02174710740')

WebUI.setText(findTestObject('CMS/Outlet/WhatsappNumber'), '08896679011')

WebUI.setText(findTestObject('CMS/Outlet/EmailOutlet'), 'outlet@outlet.com')

WebUI.click(findTestObject('CMS/Outlet/Btn_ContinueContact'))

WebUI.click(findTestObject('CMS/Outlet/Btn_OutletOpeningMonday'))

WebUI.click(findTestObject('CMS/Outlet/Btn_OutletOpeningTuesday'))

WebUI.click(findTestObject('CMS/Outlet/Btn_Open24HoursTuesday'))

WebUI.click(findTestObject('CMS/Outlet/Btn_ContinueHours'))

WebUI.waitForElementVisible(findTestObject('CMS/Outlet/UploadImage'), 5)

CustomKeywords.'tools.uploadFiles.uploadFile'(findTestObject('CMS/Outlet/UploadImage'), 'C:\\Users\\ASANI\\Pictures\\1080x606.png')

Thread.sleep(2500 //Millisecond 2.5 second delay only if needed
    )

WebUI.click(findTestObject('CMS/Outlet/Btn_Finish'))

WebUI.verifyTextPresent('New outlet has been added', true)

WebUI.closeBrowser()