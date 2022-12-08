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

Authresponse = WS.sendRequest(findTestObject('Login and Authentication/Authenticity-verification (standard code)', [('auth_token') : GlobalVariable.auth_token
            , ('authid') : GlobalVariable.authid, ('devicetype') : GlobalVariable.devicetype, ('Host') : GlobalVariable.Host
            , ('mobileNum') : GlobalVariable.mobileNum, ('bearertoken') : GlobalVariable.bearertoken, ('ucode') : GlobalVariable.ucode]))

System.out.println('Golbal value is:' + GlobalVariable.bearertoken)

WS.verifyResponseStatusCode(Authresponse, 200)

//100  ----Success message ---on scanning valid codes
//101  ---- Inactive codes--- codes which are not activated from dashboard
//102  ---  Fake products(any random code --if scanned from customer app)
//103  ---  To many attempts on code (number of attempts +3)
//If attempts is set to 4 (from dashboard --attributes)----> on 5th scan user will get response code 103
//And also for 6th and 7th
//104 --- Random codes scanned from Certify app( 104 will be sent in CERTIFY/UNIQLO LABEL    app)
//105 --- Attempts beyond limit ---which turns out to be fake product ( number of attempts +4)
//On 8th attempt user will get response code 105
//106 ---  Parameters did not match
//107 --- user not logged in
//108 --- Loyalty pop over
//109---- user not provided geo permission
//Scan code ---- when the user has below loyalty status
//		  New(108) ---->  should show loyalty pop over(loyalty apply screen)
//		  Not Approved(108) ---> should show a popover screen with comment showing
WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 100, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 105, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 103, FailureHandling.CONTINUE_ON_FAILURE)
WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 104, FailureHandling.CONTINUE_ON_FAILURE)


WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 108, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 107, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(Authresponse, 'result.response_code', 102, FailureHandling.CONTINUE_ON_FAILURE)

